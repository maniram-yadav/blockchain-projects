import { expect } from "chai";
import { ethers } from "hardhat";
import { Counter } from "../typechain-types";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";

describe("Counter", function () {
  let counter: Counter ;
  let owner: HardhatEthersSigner;

  beforeEach(async function () {
    [owner] = await ethers.getSigners();
    const TaxRecordFactory = await ethers.getContractFactory("Counter");
    counter = await TaxRecordFactory.deploy();
    await counter.waitForDeployment();
  });

 

  it("Increment counter", async function () {
    let tx = await counter.increment()
    await tx.wait();
    let count = await counter.getCount();
    expect(count).to.equal(1);
  });

  

  it("Decrement counter", async function () {
    let tx = await counter.increment()
    await tx.wait();
    let count = await counter.getCount();
    expect(count).to.be.revertedWith("Counter: underflow");
  });



});