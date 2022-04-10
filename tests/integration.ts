import axios from "axios";
import { Wallet, SecretNetworkClient, fromUtf8 } from "secretjs";
import fs from "fs";
import assert from "assert";

type EquationVariables = {
  x: string;
  y: string;
};

type UserCalculation = {
  eq: EquationVariables;
  op: string;
  res: string;
  timestamp: string;
};

type QueryAnswer = {
  status: string;
  calculations: Array<UserCalculation>;
};

type QueryAnswerWrapped = {
  get_user_calculations: QueryAnswer;
};

type CalcHandleAnswer = {
  status: string;
  res: string;
};

const initializeClient = async (endpoint: string, chain_id: string) => {
  const wallet = new Wallet();
  const accAddress = wallet.address;
  const cli = await SecretNetworkClient.create({
    grpcWebUrl: endpoint,
    chainId: chain_id,
    wallet: wallet,
    walletAddress: accAddress,
  });

  console.log(`Initialized client with wallet address: ${accAddress}`);
  return cli;
};

const initializeContract = async (
  client: SecretNetworkClient,
  contractPath: string
) => {
  const wasmCode = fs.readFileSync(contractPath);
  console.log("Uploading calc contract");

  const uploadReceipt = await client.tx.compute.storeCode(
    {
      wasmByteCode: wasmCode,
      sender: client.address,
      source: "",
      builder: "",
    },
    {
      gasLimit: 5000000,
    }
  );

  if (uploadReceipt.code !== 0) {
    console.log(
      `Failed to get code id: ${JSON.stringify(uploadReceipt.rawLog)}`
    );
    throw new Error(`Failed to upload contract`);
  }

  const codeIdKv = uploadReceipt.jsonLog![0].events[0].attributes.find(
    (a: any) => {
      return a.key === "code_id";
    }
  );

  const codeId = Number(codeIdKv!.value);
  console.log("contract codeId: ", codeId);

  const contractCodeHash = await client.query.compute.codeHash(codeId);
  console.log(`Contract hash: ${contractCodeHash}`);

  const calcInitMsg = {};
  const calcContract = await client.tx.compute.instantiateContract(
    {
      sender: client.address,
      codeId,
      initMsg: calcInitMsg,
      codeHash: contractCodeHash,
      label: "My Calc" + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 1000000,
    }
  );

  if (calcContract.code !== 0) {
    throw new Error(
      `Failed to instantiate the contract with the following error ${calcContract.rawLog}`
    );
  }

  const calcContractAddress = calcContract.arrayLog!.find(
    (log) => log.type === "message" && log.key === "contract_address"
  )!.value;

  console.log(`Calc contract address: ${calcContractAddress}`);

  var contractInfo: [string, string] = [contractCodeHash, calcContractAddress];
  return contractInfo;
};

const getFromFaucet = async (address: string) => {
  await axios.get(`http://localhost:5000/faucet?address=${address}`);
};

async function getScrtBalance(userCli: SecretNetworkClient): Promise<string> {
  let balanceResponse = await userCli.query.bank.balance({
    address: userCli.address,
    denom: "uscrt",
  });
  return balanceResponse.balance!.amount;
}

async function initializeAndUploadContract() {
  let endpoint = "http://localhost:9091";
  let chainId = "secretdev-1";

  const client = await initializeClient(endpoint, chainId);

  let balance = await getScrtBalance(client);
  let iter = 1;
  while (Number(balance) < 100000000) {
    try {
      await getFromFaucet(client.address);
    } catch (e) {
      if (iter % 100 == 0) {
        console.log(`failed to get tokens from faucet: ${e}`);
      }
      iter++;
    }
    balance = await getScrtBalance(client);
  }

  console.log(`got tokens from faucet: ${balance}`);
  const [contractHash, contractAddress] = await initializeContract(
    client,
    "../contract.wasm"
  );

  var clientInfo: [SecretNetworkClient, string, string] = [
    client,
    contractHash,
    contractAddress,
  ];
  return clientInfo;
}

async function queryContract(
  client: SecretNetworkClient,
  contractHash: string,
  calcContractAddress: string,
  cookie: string
) {
  const queryAnswer = (await client.query.compute.queryContract({
    contractAddress: calcContractAddress,
    codeHash: contractHash,
    query: { get_user_calculations: { user_cookie: cookie } },
  })) as QueryAnswerWrapped;

  return queryAnswer.get_user_calculations;
}

async function queryUserCalculations(
  client: SecretNetworkClient,
  contractHash: string,
  contractAddress: string
): Promise<number> {
  const queryAnswer = await queryContract(
    client,
    contractHash,
    contractAddress,
    client.address
  );
  if ("generic_err" in queryAnswer) {
    throw new Error(
      `Query failed with the following err: ${JSON.stringify(queryAnswer)}`
    );
  }

  console.log(
    `Query status was ${queryAnswer.status} and length was ${queryAnswer.calculations.length}`
  );

  return queryAnswer.calculations.length;
}

async function addTx(
  client: SecretNetworkClient,
  contractHash: string,
  calcContractAddess: string
) {
  let x: number = 4;
  let y: number = 8;
  let eq: EquationVariables = { x: String(x), y: String(y) }; // cosmwasm sdk's Uint128 type is serialized as a string in the json
  const tx = await client.tx.compute.executeContract(
    {
      sender: client.address,
      contractAddress: calcContractAddess,
      codeHash: contractHash,
      msg: {
        add: { eq: eq },
      },
      sentFunds: [],
    },
    {
      gasLimit: 200000,
    }
  );

  let ans: CalcHandleAnswer = JSON.parse(fromUtf8(tx.data[0])).add;
  assert(
    Number(ans.res) === x + y,
    `Calculation answer of function \"Add\" was incorrect expected ${
      x + y
    } got ${ans.res}`
  );

  console.log(`Add TX used ${tx.gasUsed} gas`);

  return ans;
}

async function subTx(
  client: SecretNetworkClient,
  contractHash: string,
  calcContractAddess: string
) {
  let x: number = 8;
  let y: number = 4;
  let eq: EquationVariables = { x: String(x), y: String(y) }; // cosmwasm sdk's Uint128 type is serialized as a string in the json
  const tx = await client.tx.compute.executeContract(
    {
      sender: client.address,
      contractAddress: calcContractAddess,
      codeHash: contractHash,
      msg: {
        sub: { eq: eq },
      },
      sentFunds: [],
    },
    {
      gasLimit: 200000,
    }
  );

  let ans: CalcHandleAnswer = JSON.parse(fromUtf8(tx.data[0])).sub;
  assert(
    Number(ans.res) === x - y,
    `Calculation answer of function \"Sub\" was incorrect expected ${
      x - y
    } got ${ans.res}`
  );
  console.log(`Sub TX used ${tx.gasUsed} gas`);

  return ans;
}

async function mulTx(
  client: SecretNetworkClient,
  contractHash: string,
  calcContractAddess: string
) {
  let x: number = 8;
  let y: number = 4;
  let eq: EquationVariables = { x: String(x), y: String(y) }; // cosmwasm sdk's Uint128 type is serialized as a string in the json
  const tx = await client.tx.compute.executeContract(
    {
      sender: client.address,
      contractAddress: calcContractAddess,
      codeHash: contractHash,
      msg: {
        mul: { eq: eq },
      },
      sentFunds: [],
    },
    {
      gasLimit: 200000,
    }
  );

  let ans: CalcHandleAnswer = JSON.parse(fromUtf8(tx.data[0])).mul;
  assert(
    Number(ans.res) === x * y,
    `Calculation answer of function \"Mul\" was incorrect expected ${
      x * y
    } got ${ans.res}`
  );
  console.log(`Mul TX used ${tx.gasUsed} gas`);

  return ans;
}

async function divTx(
  client: SecretNetworkClient,
  contractHash: string,
  calcContractAddess: string
) {
  let x: number = 8;
  let y: number = 3;
  let eq: EquationVariables = { x: String(x), y: String(y) }; // cosmwasm sdk's Uint128 type is serialized as a string in the json
  const tx = await client.tx.compute.executeContract(
    {
      sender: client.address,
      contractAddress: calcContractAddess,
      codeHash: contractHash,
      msg: {
        div: { eq: eq },
      },
      sentFunds: [],
    },
    {
      gasLimit: 200000,
    }
  );

  let ans: CalcHandleAnswer = JSON.parse(fromUtf8(tx.data[0])).div;
  assert(
    Number(ans.res) === 2.666,
    `Calculation answer of function \"Div\" was incorrect expected 2.333 got ${ans.res}`
  );
  console.log(`Div TX used ${tx.gasUsed} gas`);

  return ans;
}

async function sqrtTx(
  client: SecretNetworkClient,
  contractHash: string,
  calcContractAddess: string
) {
  let x: number = 8;
  const tx = await client.tx.compute.executeContract(
    {
      sender: client.address,
      contractAddress: calcContractAddess,
      codeHash: contractHash,
      msg: {
        sqrt: { x: String(x) },
      },
      sentFunds: [],
    },
    {
      gasLimit: 200000,
    }
  );

  let ans: CalcHandleAnswer = JSON.parse(fromUtf8(tx.data[0])).sqrt;
  assert(
    Number(ans.res) === 2.828,
    `Calculation answer of function \"Sqrt\" was incorrect expected 2.828 got ${ans.res}`
  );
  console.log(`Sqrt TX used ${tx.gasUsed} gas`);

  return ans;
}

async function test_zero_calculations_on_initialization(
  client: SecretNetworkClient,
  contractHash: string,
  contractAddress: string
) {
  assert(
    (await queryUserCalculations(client, contractHash, contractAddress)) === 0
  );
}

async function test_calculation_count_is_raised(
  client: SecretNetworkClient,
  contractHash: string,
  contractAddress: string
) {
  const prevCount = await queryUserCalculations(
    client,
    contractHash,
    contractAddress
  );

  await addTx(client, contractHash, contractAddress);
  await subTx(client, contractHash, contractAddress);
  await mulTx(client, contractHash, contractAddress);
  await divTx(client, contractHash, contractAddress);
  await sqrtTx(client, contractHash, contractAddress);

  const currentCount = await queryUserCalculations(
    client,
    contractHash,
    contractAddress
  );
  assert(
    prevCount + 5 === currentCount,
    `After sending 5 transactions, expected the amount of user calculations to grow by 5. Previous amount is: ${prevCount}, current amount is: ${currentCount}`
  );
}

async function test_calculation_load(
  client: SecretNetworkClient,
  contractHash: string,
  contractAddress: string
) {
  let load = 10;
  const prevCount = await queryUserCalculations(
    client,
    contractHash,
    contractAddress
  );

  for (let i = 0; i < load; ++i) {
    await sqrtTx(client, contractHash, contractAddress);
  }

  const currentCount = await queryUserCalculations(
    client,
    contractHash,
    contractAddress
  );

  assert(
    prevCount + load === currentCount,
    `After sending ${load} Sqrt transactions, expected the amount of user calculations to grow by ${load}. Previous amount is: ${prevCount}, current amount is: ${currentCount}`
  );
}

async function test_gas_limits() {
  // There is no accurate way to measue gas limits but it is actually very recommended to make sure that the gas that is used by a specific tx makes sense
}

async function runTestFunction(
  tester: (
    client: SecretNetworkClient,
    contractHash: string,
    contractAddress: string
  ) => void,
  client: SecretNetworkClient,
  contractHash: string,
  contractAddress: string
) {
  console.log(`Testing ${tester.name}`);
  await tester(client, contractHash, contractAddress);
  console.log(`[SUCCESS] ${tester.name}`);
}

(async () => {
  const [client, contractHash, contractAddress] =
    await initializeAndUploadContract();

  await test_zero_calculations_on_initialization(
    client,
    contractHash,
    contractAddress
  );

  await runTestFunction(
    test_calculation_count_is_raised,
    client,
    contractHash,
    contractAddress
  );
  await runTestFunction(
    test_calculation_load,
    client,
    contractHash,
    contractAddress
  );
  await runTestFunction(test_gas_limits, client, contractHash, contractAddress);
})();
