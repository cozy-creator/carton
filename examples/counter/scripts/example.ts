import { carton } from "sui-carton";
import { SuiMoveObject, SuiObject, SuiObjectInfo } from "@mysten/sui.js";

const counter = "0x89c532c55cada85287d0d19b403ebbd334df8a0c";

async function getCounter(counterId: string) {
  const object = await carton.provider.getObject(counterId);
  if (object.status !== "Exists") {
    throw new Error("Counter not found");
  }

  const { data } = object.details as SuiObject;
  return (data as SuiMoveObject).fields;
}

getCounter(counter)
  .then((fields) => {
    console.log("Counter ID is %s", fields.id.id);
    console.log("Counter value is %s", fields.value);
  })
  .catch(console.log);
