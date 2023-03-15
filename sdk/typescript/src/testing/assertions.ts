// import tap from "tap";
// import { isValidSuiAddress, isValidSuiObjectId, normalizeSuiAddress, SuiAddress, SuiObjectInfo } from "@mysten/sui.js";

// tap.Test.prototype.addAssert("isValidSuiAddress", 1, (value: string, message?: string, extra?: any) => {
//   message = message || "should be a valid address";

//   return tap.ok(isValidSuiAddress(value), message, extra);
// });

// tap.Test.prototype.addAssert("isValidSuiObjectId", 1, (value: string, message?: string, extra?: any) => {
//   message = message || "should be a valid Sui object ID";

//   return tap.ok(isValidSuiObjectId(value), message, extra);
// });

// tap.Test.prototype.addAssert(
//   "isObjectOwner",
//   2,
//   (address: SuiAddress, object: SuiObjectInfo, message?: string, extra?: any) => {
//     message = message || "should be object owner";
//     address = normalizeSuiAddress(address);

//     // @ts-expect-error
//     let owner = normalizeSuiAddress(object.owner.AddressOwner || object.owner.ObjectOwner);

//     return tap.equal(address, owner, message, extra);
//   }
// );

// tap.Test.prototype.addAssert("isTypeOf", 2, (object: SuiObjectInfo, type: string, message?: string, extra?: any) => {
//   message = message || `should be a type of ${type}`;

//   return tap.equal(object.type, type, message, extra);
// });

// tap.Test.prototype.addAssert("isSharedObject", 1, (object: SuiObjectInfo, message?: string, extra?: any) => {
//   message = message || "should be a shared object";

//   return tap.ok(Object.keys(object.owner).includes("Shared"), message, extra);
// });

// tap.Test.prototype.addAssert("isImmutableObject", 1, (object: SuiObjectInfo, message?: string, extra?: any) => {
//   message = message || "should be an immutable object";

//   return tap.equal(object.owner, "Immutable", message, extra);
// });
