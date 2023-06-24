// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.12;

interface IERC20 {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
    function allowance(address owner, address spender) external view returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
}

interface IMessageRecipient {
    /**
     * @notice Handle an interchain message
     * @param _origin Domain ID of the chain from which the message came
     * @param _sender Address of the message sender on the origin chain as bytes32
     * @param _body Raw bytes content of message body
     */
    function handle(
        uint32 _origin,
        bytes32 _sender,
        bytes calldata _body
    ) external;
}


contract AccountEscrow is IMessageRecipient {
/** This escrow contract (Account Escrow) is a singleton contract
  * The purpose of this contract is to be the intermediary contract
  * for storing account funds (tokens and ETH) to bribe paymasters 
  * within the 4337 ecosystem to pay for autonomously executing
  * transactions on any EVM chain
  *
  * To support this functionality this contract must
  *     - map account to funds
  *     - escrow must have a freeze period
  *     - payback amount and rceipient must be deterministic
  *     - must have a handle function to accept messages from hyperlane mailbox
  *
  * Simplest solution to payback
  * userop calldata has target address, original transactions data, and signature
  * to reconstruct the signer (ie signer == target)
  */
    mapping(address => Escrow) accountInfo;
    mapping(address => bool) entryPoint;

    struct Escrow {
        uint256 freezeStart;
        uint256 freezeStop;
        uint256 nonce;
        mapping(address => uint256) balance;
        mapping(uint256 => Payment) history;
    }

    struct Payment {
        uint256 timestamp;
        uint256 assetAmount;
        bytes32 domainId;
        bytes32 hashId; // tba for contenuity challenging
        address from;
        address asset;
    }

    bool lock;
    modifier locked() {
        require(lock, "no reentry");
        lock = true;
        _;
        lock = false;
    }

    address constant mailbox = 0x0E3239277501d215e17a4d31c487F86a425E110B;
    // for access control on handle implementations
    modifier onlyMailbox() {
        require(msg.sender == mailbox);
        _;    
    }

    function handle(
        uint32 _origin,
        bytes32 _sender,
        bytes calldata _body
    ) external onlyMailbox {
        require(_body.length >= 84, "Insufficent payout data");)
        address _account = _body[:20];
        bytes32 _hash = _body[20:52];
        bytes32 _signature = body[52:84];
        this.payout(_origin, _sender, _account, _hash, _signature)
    }

    function payout(
        uint32 _origin, 
        address _sender, 
        address _account, 
        bytes32 _hash,
        bytes32 _signature) external {
            require(entryPoint[address(_sender)], "Not a valid entryPoint");
        }

    // for now no withdraw function for the user
    // but this will be required to be from a userop (ie from entrypoint)
    // thought is that if for the transaction the user is their own paymaster

    // ETH balance is address(0)
    function deposit(address asset_, uint256 amount_) external payable locked() {
        address account = msg.sender;
        Escrow storage accountInfo_ = accountInfo[account];

        if(asset_ == address(0)) { 
            require(amount_ == msg.value, "Insufficent ETH deposit");
        } else {
            require(IERC20(asset_).balanceOf(msg.sender) >= amount_, "Insufficent ERC20 balance");
        }

        accountInfo_.freezeStart = block.timestamp;
        accountInfo_.freezeStop = block.timestamp + 3600;
        accountInfo_.balance[asset_] += amount_;
    }

    receive() external payable {}
}