const assert = require("node:assert");
const { carton } = require("sui-carton");

const packageObjectId = "0x437bd6eff3eacd3b6e98d549c69402564135effe";
const counter = "0x89c532c55cada85287d0d19b403ebbd334df8a0c";

describe("Counter tests", function () {
  this.timeout(15000);

  it("increments counter", async function () {
    const tx = await carton.signer.executeMoveCall({
      packageObjectId,
      module: "counter",
      function: "increment",
      arguments: [counter],
      typeArguments: [],
      gasBudget: 30000,
    });

    assert.equal(tx.effects.effects.status.status, "success", "Increment successful");
  });

  it("decrements counter", async function () {
    const tx = await carton.signer.executeMoveCall({
      packageObjectId,
      module: "counter",
      function: "decrement",
      arguments: [counter],
      typeArguments: [],
      gasBudget: 30000,
    });

    assert.equal(tx.effects.effects.status.status, "success", "Decrement successful");
  });
});
