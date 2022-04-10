var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var _this = this;
var axios = require("axios");
var _a = require("secretjs"), Wallet = _a.Wallet, SecretNetworkClient = _a.SecretNetworkClient;
var fs = require("fs");
var initializeCLI = function (endpoint, chain_id) { return __awaiter(_this, void 0, void 0, function () {
    var wallet, accAddress, cli;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                wallet = new Wallet();
                accAddress = wallet.address;
                return [4 /*yield*/, SecretNetworkClient.create({
                        grpcWebUrl: endpoint,
                        chainId: chain_id,
                        wallet: wallet,
                        walletAddress: accAddress
                    })];
            case 1:
                cli = _a.sent();
                console.log(accAddress);
                console.log(cli.address);
                return [2 /*return*/, cli];
        }
    });
}); };
function queryContract(client, contractHash, calcContractAddress, cookie) {
    return __awaiter(this, void 0, void 0, function () {
        var queryAnswer;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, client.query.compute.queryContract({
                        contractAddress: calcContractAddress,
                        codeHash: contractHash,
                        query: { get_user_calculations: { user_cookie: cookie } }
                    })];
                case 1:
                    queryAnswer = (_a.sent());
                    return [2 /*return*/, queryAnswer.get_user_calculations];
            }
        });
    });
}
function addTx(client, contractHash, calcContractAddess, x, y) {
    return __awaiter(this, void 0, void 0, function () {
        var eq, addAnswer;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    eq = { x: x, y: y };
                    console.log("here");
                    return [4 /*yield*/, client.tx.compute.executeContract({
                            sender: client.address,
                            contractAddress: calcContractAddess,
                            codeHash: contractHash,
                            msg: {
                                add: { eq: eq }
                            },
                            sentFunds: []
                        }, {
                            gasLimit: 200000
                        })];
                case 1:
                    addAnswer = (_a.sent());
                    console.log(JSON.stringify(addAnswer, null, 4));
                    return [2 /*return*/, addAnswer.add];
            }
        });
    });
}
var getFromFaucet = function (address) { return __awaiter(_this, void 0, void 0, function () {
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                console.log("address=" + address);
                return [4 /*yield*/, axios.get("http://localhost:5000/faucet?address=" + address)];
            case 1:
                _a.sent();
                return [2 /*return*/];
        }
    });
}); };
function getScrtBalance(userCli) {
    return __awaiter(this, void 0, void 0, function () {
        var balanceResponse;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, userCli.query.bank.balance({
                        address: userCli.address,
                        denom: "uscrt"
                    })];
                case 1:
                    balanceResponse = _a.sent();
                    return [2 /*return*/, balanceResponse.balance.amount];
            }
        });
    });
}
(function () { return __awaiter(_this, void 0, void 0, function () {
    var endpoint, chainId, secretNetwork, balance, e_1, wasmCode, t, uploadReceipt, codeIdKv, codeId, contractCodeHash, calcInitMsg, calcContract, calcContractAddress, queryAnswer, addResponse, query2Answer;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                endpoint = "http://localhost:9091";
                chainId = "secretdev-1";
                return [4 /*yield*/, initializeCLI(endpoint, chainId)];
            case 1:
                secretNetwork = _a.sent();
                return [4 /*yield*/, getScrtBalance(secretNetwork)];
            case 2:
                balance = _a.sent();
                _a.label = 3;
            case 3:
                if (!(Number(balance) < 100000000)) return [3 /*break*/, 9];
                _a.label = 4;
            case 4:
                _a.trys.push([4, 6, , 7]);
                return [4 /*yield*/, getFromFaucet(secretNetwork.address)];
            case 5:
                _a.sent();
                return [3 /*break*/, 7];
            case 6:
                e_1 = _a.sent();
                console.log("failed to get tokens from faucet: " + e_1);
                return [3 /*break*/, 7];
            case 7: return [4 /*yield*/, getScrtBalance(secretNetwork)];
            case 8:
                balance = _a.sent();
                return [3 /*break*/, 3];
            case 9:
                console.log("got tokens from faucet: " + balance);
                wasmCode = fs.readFileSync("./contract.wasm");
                console.log("Uploading calc contract");
                return [4 /*yield*/, secretNetwork.query.registration.txKey({})];
            case 10:
                t = _a.sent();
                return [4 /*yield*/, secretNetwork.tx.compute.storeCode({
                        wasmByteCode: wasmCode,
                        sender: secretNetwork.address,
                        source: "",
                        builder: ""
                    }, {
                        gasLimit: 5000000
                    })];
            case 11:
                uploadReceipt = _a.sent();
                if (uploadReceipt.code !== 0) {
                    console.log("Failed to get code id: " + JSON.stringify(uploadReceipt.rawLog));
                    throw new Error("Failed to upload contract");
                }
                codeIdKv = uploadReceipt.jsonLog[0].events[0].attributes.find(function (a) {
                    return a.key === "code_id";
                });
                codeId = Number(codeIdKv.value);
                console.log("codeId: ", codeId);
                return [4 /*yield*/, secretNetwork.query.compute.codeHash(codeId)];
            case 12:
                contractCodeHash = _a.sent();
                console.log("Contract hash: " + contractCodeHash);
                calcInitMsg = {};
                return [4 /*yield*/, secretNetwork.tx.compute.instantiateContract({
                        sender: secretNetwork.address,
                        codeId: codeId,
                        initMsg: calcInitMsg,
                        codeHash: contractCodeHash,
                        label: "My Calc" + Math.ceil(Math.random() * 10000)
                    }, {
                        gasLimit: 1000000
                    })];
            case 13:
                calcContract = _a.sent();
                if (calcContract.code != 0) {
                    throw new Error("Failed to instantiate the contract with the following error " + calcContract.rawLog);
                }
                calcContractAddress = calcContract.arrayLog.find(function (log) { return log.type === "message" && log.key === "contract_address"; }).value;
                console.log("Calc contract address: " + calcContractAddress);
                return [4 /*yield*/, queryContract(secretNetwork, contractCodeHash, calcContractAddress, secretNetwork.address)];
            case 14:
                queryAnswer = _a.sent();
                if ("generic_err" in queryAnswer) {
                    throw new Error("Query failed with the following err: " + JSON.stringify(queryAnswer));
                }
                console.log("Query failed with the following err: " + JSON.stringify(queryAnswer));
                console.log("Query status was " + queryAnswer.status + " and length was " + queryAnswer.calculations.length);
                return [4 /*yield*/, addTx(secretNetwork, contractCodeHash, calcContractAddress, "4", "8")];
            case 15:
                addResponse = _a.sent();
                return [4 /*yield*/, queryContract(secretNetwork, contractCodeHash, calcContractAddress, secretNetwork.address)];
            case 16:
                query2Answer = _a.sent();
                console.log("Query status was " + query2Answer.status + " and length was " + query2Answer.calculations.length);
                return [2 /*return*/];
        }
    });
}); })();
