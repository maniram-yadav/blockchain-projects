// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

contract Counter {
    uint256 public count;

    function increment() public {
        count += 1;
    }

    function decrement() public {
        require(count > 0, "Counter: underflow");
        count -= 1;
    }

    function reset() public {
        count = 0;
    }

    function getCount() public view returns (uint256) {
        return count;
    }
}
