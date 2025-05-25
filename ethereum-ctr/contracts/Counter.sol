// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract Counter {
    uint256 private count;
    address public owner;

    event CountIncremented(uint256 newCount);
    event CountDecremented(uint256 newCount);
    event CountReset(uint256 newCount);

    constructor() {
        owner = msg.sender;
        count = 0;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    function getCount() public view returns (uint256) {
        return count;
    }

    function increment() public onlyOwner {
        count += 1;
        emit CountIncremented(count);
    }

    function decrement() public onlyOwner {
        require(count > 0, "Counter: cannot decrement below zero");
        count -= 1;
        emit CountDecremented(count);
    }

    function reset() public onlyOwner {
        count = 0;
        emit CountReset(count);
    }
}