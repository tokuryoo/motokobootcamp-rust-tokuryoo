import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent } from "@dfinity/agent";

import { createActor, canisterId} from "../../declarations/day6_backend";
var calculatorAgent;

async function createCalculatorActor() {
  console.log("createCalculatorActor debug1")
  // 参考 https://forum.dfinity.org/t/dfx-0-12-1-importing-createactor-dfx-generate-do-not-import-from-dfx-when-using-react/16648
  const identity = await authClient.getIdentity();
  // createActor は、メインメット上では正常に動作した。
  // ローカルではエラーになる。おそらくは、メインネット上の II を利用しているため。
  const myActor = createActor(canisterId, { agentOptions: { identity } });
  console.log("createCalculatorActor debug2")
  return myActor;
};

async function initCalculatorAgent() {
  if (calculatorAgent == null) {
    calculatorAgent = await createCalculatorActor();
    console.log("calculatorAgent has been initialized.");
  }
}

document.getElementById("add").onclick = async () => {
  await initCalculatorAgent();
  console.log("add debugXXX", calculatorAgent);
  const value = Number.parseFloat(document.getElementById("value").value);
  document.getElementById("current").innerText = "処理中";
  const result = await calculatorAgent.add(value);
  document.getElementById("current").innerText = result;
};

document.getElementById("sub").onclick = async () => {
  await initCalculatorAgent();
  console.log("sub debugX", calculatorAgent);
  const value = Number.parseFloat(document.getElementById("value").value);
  document.getElementById("current").innerText = "処理中";
  const result = await calculatorAgent.sub(value);
  document.getElementById("current").innerText = result;
};

document.getElementById("mul").onclick = async () => {
  await initCalculatorAgent();
  const value = Number.parseFloat(document.getElementById("value").value);
  document.getElementById("current").innerText = "処理中";
  const result = await calculatorAgent.mul(value);
  document.getElementById("current").innerText = result;
};

document.getElementById("div").onclick = async () => {
  await initCalculatorAgent();
  const value = Number.parseFloat(document.getElementById("value").value);
  document.getElementById("current").innerText = "処理中";
  const result = await calculatorAgent.div(value);
  document.getElementById("current").innerText = result;
};

document.getElementById("power").onclick = async () => {
  await initCalculatorAgent();
  const value = Number.parseFloat(document.getElementById("value").value);
  document.getElementById("current").innerText = "処理中";
  const result = await calculatorAgent.power(value);
  document.getElementById("current").innerText = result;
};

document.getElementById("sqrt").onclick = async () => {
  await initCalculatorAgent();
  document.getElementById("current").innerText = "処理中";
  const result = await calculatorAgent.sqrt();
  document.getElementById("current").innerText = result;
};

document.getElementById("floor").onclick = async () => {
  await initCalculatorAgent();
  document.getElementById("current").innerText = "処理中";
  const result = await calculatorAgent.floor();
  document.getElementById("current").innerText = result;
};

document.getElementById("reset").onclick = async () => {
  await initCalculatorAgent();
  document.getElementById("current").innerText = "処理中";
  await calculatorAgent.reset();
  document.getElementById("current").innerText = 0.0;
};

var authClient;

document.getElementById("login").onclick = async () => {
  console.log("login start");
  if (authClient == null) {
    // https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/integrate-identity/#using-the-auth-client-library-to-log-in-with-internet-identity
    authClient = await AuthClient.create();
    await new Promise((resolve, reject) => {
      authClient.login({
        // ローカル用
        // TODO ハードコードしない
        // 参考 https://github.com/dfinity/examples/tree/master/motoko/internet_identity_integration
        // internet_identity のフロントエンドの id
        identityProvider: "http://127.0.0.1:4943/?canisterId=a3shf-5eaaa-aaaaa-qaafa-cai",
        onSuccess: async () => {
          const authenticated = await authClient.isAuthenticated();
          console.log("login - authenticated", authenticated);
          console.log("Principal ID : ", authClient.getIdentity().getPrincipal().toString());
          document.getElementById("login").textContent = "Logout"
          // 現在の値を取得して表示（前回ログイン時の計算結果が残っているため）
          await initCalculatorAgent();
          document.getElementById("current").innerText = "取得中";
          const result = await calculatorAgent.see();
          document.getElementById("current").innerText = result;
        },
        onError: async () => {
          console.log("login error");
        },
      });
    });
  } else {
    if (authClient.isAuthenticated()) {
      await authClient.logout();
      const authenticated = await authClient.isAuthenticated();
      console.log("logout - authenticated", authenticated);
      document.getElementById("login").textContent = "Login"
      // TODO null を代入しないほうが、より良い実装かもしれない？ログイン/ログアウト周りは、とりあえず動けばよいというレベル感で実装しており、何か細かいバグはあるかもしれない。
      authClient = null;
    };
  }
};