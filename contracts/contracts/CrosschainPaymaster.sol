// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.12;

// Import the required libraries and contracts

import { EntryPoint } from "./EntryPoint_flat.sol";
import { IEntryPoint, BasePaymaster } from "./BasePaymaster_flat.sol";
import "./UserOperation.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @title IInterchainGasPaymaster
 * @notice Manages payments on a source chain to cover gas costs of relaying
 * messages to destination chains.
 */
interface IInterchainGasPaymaster {
    /**
     * @notice Emitted when a payment is made for a message's gas costs.
     * @param messageId The ID of the message to pay for.
     * @param gasAmount The amount of destination gas paid for.
     * @param payment The amount of native tokens paid.
     */
    event GasPayment(
        bytes32 indexed messageId,
        uint256 gasAmount,
        uint256 payment
    );

    /**
     * @notice Deposits msg.value as a payment for the relaying of a message
     * to its destination chain.
     * @dev Overpayment will result in a refund of native tokens to the _refundAddress.
     * Callers should be aware that this may present reentrancy issues.
     * @param _messageId The ID of the message to pay for.
     * @param _destinationDomain The domain of the message's destination chain.
     * @param _gasAmount The amount of destination gas to pay for.
     * @param _refundAddress The address to refund any overpayment to.
     */
    function payForGas(
        bytes32 _messageId,
        uint32 _destinationDomain,
        uint256 _gasAmount,
        address _refundAddress
    ) external payable;

    /**
     * @notice Quotes the amount of native tokens to pay for interchain gas.
     * @param _destinationDomain The domain of the message's destination chain.
     * @param _gasAmount The amount of destination gas to pay for.
     * @return The amount of native tokens required to pay for interchain gas.
     */
    function quoteGasPayment(uint32 _destinationDomain, uint256 _gasAmount)
        external
        view
        returns (uint256);
}

interface IMailbox {
    function dispatch(
        uint32 _destinationDomain,
        bytes32 _recipientAddress,
        bytes calldata _messageBody
    ) external returns (bytes32);

    function process(bytes calldata _metadata, bytes calldata _message)
        external;
}

/// @title ross-chain Paymaster for ERC-4337
/// @notice Paymaster to send crosschain payments
/// @dev Inherits from BasePaymaster.
contract CrosschainPaymaster is BasePaymaster {

    struct TokenPaymasterConfig {
        /// @notice The price markup percentage applied to the token price (1e6 = 100%)
        uint256 priceMarkup;

        /// @notice Exchange tokens to native currency if the EntryPoint balance of this Paymaster falls below this value
        uint256 minEntryPointBalance;

        /// @notice Estimated gas cost for refunding tokens after the transaction is completed
        uint256 refundPostopCost;

        /// @notice Transactions are only valid as long as the cached price is not older than this value
        uint256 priceMaxAge;
    }

    event ConfigUpdated(TokenPaymasterConfig tokenPaymasterConfig);

    event UserOperationSponsored(address indexed user, uint256 actualTokenCharge, uint256 actualGasCost, uint256 actualTokenPrice);

    event PostOpReverted(address indexed user, uint256 preCharge);

    event Received(address indexed sender, uint256 value);

    /// @notice All 'price' variables are multiplied by this value to avoid rounding up
    uint256 private constant PRICE_DENOMINATOR = 1e26;

    TokenPaymasterConfig private tokenPaymasterConfig;

    // TODO: I don't like defaults in Solidity - accept ALL parameters of fail!!!
    /// @notice Initializes the PimlicoERC20Paymaster contract with the given parameters.
    /// @param _entryPoint The EntryPoint contract used in the Account Abstraction infrastructure.
    /// @param _owner The address that will be set as the owner of the contract.
    constructor(
        address _entryPoint,
        address _owner
    ) BasePaymaster(IEntryPoint(_entryPoint))
    {
        transferOwnership(_owner);
    }


    /// @notice Validates a paymaster user operation and calculates the required token amount for the transaction.
    /// @param userOp The user operation data.
    /// @param requiredPreFund The amount of tokens required for pre-funding.
    /// @return context The context containing the token amount and user sender address (if applicable).
    /// @return validationResult A uint256 value indicating the result of the validation (always 0 in this implementation).
    function _validatePaymasterUserOp(UserOperation calldata userOp, bytes32, uint256 requiredPreFund)
    internal
    override
    returns (bytes memory context, uint256 validationResult) {unchecked {
        // send with hash and signature
        uint256 paymasterAndDataLength = userOp.paymasterAndData.length;
        require(paymasterAndDataLength == 0 || paymasterAndDataLength < 72,
            "TPM: invalid data length"
        );

        // require domain 
        // require assets address
        // require payment amount (from chain A)

        // TODO: validate data:
        // address _account;
        // address _asset;
        // uint256 _amount;
        // bytes calldata

        bytes memory extractedData = new bytes(72);
        for (uint256 i = 0; i < 72; i++) {
            extractedData[i] = userOp.paymasterAndData[i + 20];
        }

        // TODO: add the dmain to calldata
        //uint32 domain               = 5; // goerli
        uint32 domain               = 80001; // mumbai
        address ethereumMailbox     = 0xCC737a94FecaeC165AbCf12dED095BB13F037685; // same on all chains
        address interchainPaymaster = 0xF90cB82a76492614D07B82a7658917f3aC811Ac1; // same on all chains
        address accountEscrow        = 0x0000000000000000000000000000000000000000;
        bytes32 dummyData;
        bytes memory bribeRequest = abi.encodePacked(
            extractedData, // account, asset, amount
            dummyData, // keccakHash of calldata, for proof
            dummyData, // signature
            userOp.callData
            );
        bytes32 messageId = IMailbox(ethereumMailbox).dispatch(domain, addressToBytes32(accountEscrow), bribeRequest);
        uint256 gasQuote = IInterchainGasPaymaster(interchainPaymaster).quoteGasPayment(domain, 150000);
        // TODO: change interchain gas payment to be surced from paymaster deposit
        //      and for refund to return there as well
        IInterchainGasPaymaster(interchainPaymaster).payForGas(messageId, domain, gasQuote, address(this)); // assumes funds directly on paymaster
        
    }}

    /// @notice Performs post-operation tasks,
    /// @dev This function is called after a user operation has been executed or reverted.
    /// @param mode The post-operation mode (either successful or reverted).
    /// @param context The context containing the token amount and user sender address.
    /// @param actualGasCost The actual gas cost of the transaction.
    function _postOp(PostOpMode mode, bytes calldata context, uint256 actualGasCost) internal override {
        // not nedded, but required to exist
    }

    function getGasPrice(uint256 maxFeePerGas, uint256 maxPriorityFeePerGas) internal view returns (uint256) {
        if (maxFeePerGas == maxPriorityFeePerGas) {
            //legacy mode (for networks that don't support basefee opcode)
            return maxFeePerGas;
        }
        return min(maxFeePerGas, maxPriorityFeePerGas + block.basefee);
    }

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    // alignment preserving cast
    function addressToBytes32(address _addr) internal pure returns (bytes32) {
        return bytes32(uint256(uint160(_addr)));
    }


    receive() external payable {
        emit Received(msg.sender, msg.value);
    }
}
