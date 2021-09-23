const sync = require("yare-sync");
const path = require("path");
const os = require("os");
const fs = require("fs");

require("colors");

function input() {
  return new Promise((r) => {
    process.stdin.once("data", (d) => {
      r(d);
    });
  });
}

function login() {
  return new Promise(async (resolve) => {
    console.log("Log in to yare to enable yare-sync".bold);
    console.log("Username:");
    let username = ((await input()) + "").split("\n")[0].split("\r")[0];
    console.log("Password (SHOWN):");
    let password = ((await input()) + "").split("\n")[0].split("\r")[0];
    console.log("Trying to log in as".yellow, username);
    let acc = sync.login(username, password).catch(async (e) => {
      console.log("Invalid username or password, try again".red.bold);
      resolve(await login());
    });
    if (acc) resolve(acc);
  });
}

async function main() {
  let acc;
  let savedSessionFilePath = path.join(
    os.tmpdir(),
    "yare-sync-last-session.json"
  );
  if (fs.existsSync(savedSessionFilePath)) {
    let savedSessionFile = JSON.parse(
      fs.readFileSync(savedSessionFilePath, "utf-8")
    );
    console.log("Found previous session".blue);
    if (sync.verifySession(savedSessionFile)) {
      console.log("Session was valid! Using that".green);
      acc = savedSessionFile;
    } else {
      console.log("Invalid session".red);
    }
  }
  if (acc === null) {
    acc = await login();
  }
  console.log("Logged in as".green.bold, acc.user_id, "\n");
  fs.writeFileSync(savedSessionFilePath, JSON.stringify(acc), "utf-8");

  let code = fs.readFileSync("./build/bundle.js", "utf-8");

  console.log("Getting live games...".yellow);
  let games = await sync.getGames(acc.user_id);
  console.log(
    `Uploading to these games:`.yellow,
    games.map((g) => (g ? `${g.server}/${g.id}` : g))
  );
  let successful = await sync.sendCode(code, games, acc);
  if (successful) {
    console.log("Done!".green.bold);
  } else {
    console.error("Upload to yare failed.".red.bold);
  }
}
main();
