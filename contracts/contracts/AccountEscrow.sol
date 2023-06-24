// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.12;

contract AccountEscrow {
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
    address from;
    address asset;
  }


}