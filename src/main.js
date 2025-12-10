// When using the Tauri API npm package:
//import { invoke } from '@tauri-apps/api/core';

// When using the Tauri global script (if not using the npm package)
// Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
const { invoke } = window.__TAURI__.core;

// Utility function to get invoke function safely
//function getInvoke() {
//  if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
//    return window.__TAURI__.core.invoke;
//  }
//  throw new Error('Tauri API not available. Make sure Tauri script is loaded.');
//}

function setCurrentUser(user) {
  sessionStorage.setItem("currentUser", JSON.stringify(user));
}

function getCurrentUser() {
  const raw = sessionStorage.getItem("currentUser");
  return raw ? JSON.parse(raw) : null;
}

async function createAccount(username, password) {
//  const invoke = getInvoke();
  const resultCode = await invoke("save_account", { username, password });
  let message = "";
  switch (resultCode) {
    case 0:
      message = "Account created successfully.";
      break;
    case 1:
      message = "Error: Failed to create data directory.";
      break;
    case 2:
      message = "Error: Account already exists.";
      break;
    default:
      message = "An unknown error occurred.";
  }
  
  console.log("Create Account: ", message);
}

async function login(username, password) {
//  const invoke = getInvoke();
  const isValid = await invoke("validate_login", { username, password });
  if (isValid === true) {
    setCurrentUser(username);
    console.log("Login: ", username, " logged in.");
    return true;
  } else {
    let message = "Invalid username or password.";
    console.log("Login: ", message);
  }

  return false;
}

async function getData(username) {
//  const invoke = getInvoke();
  const data = await invoke("get_data", { username });
  return data;
}

async function saveData(username, service, account, password) {
//  const invoke = getInvoke();
  const resultCode = await invoke("save_data", { username, service, account, password });
  let message = "";
  switch (resultCode) {
    case 0:
      message = "Data saved successfully.";
      return true;
    case 2:
      message = "Error: User data file does not exist.";
      break;
    case 3:
      message = "Error: User data file not found.";
      break;
    case 4:
      message = "Error: Failed to write data to file.";
      break;
    case 5:
      message = "Error: Duplicate entry for the same service and account.";
      break;
    case 6:
      message = "Error: Data encryption failed.";
      break;
    case 7:
      message = "Error: Data decryption failed.";
      break;
    default:
      message = "An unknown error occurred.";
  }
  
  console.log("Save Data Result Message: ", message);
  return false;
}

async function removeData(username, service) {
//  const invoke = getInvoke();
  const resultCode = await invoke("remove_data", { username, service });
  let message = "";
  switch (resultCode) {
    case 0:
      message = "Data removed successfully.";
      console.log("Remove Data Result Message: ", message);
      return true;
    case 2:
      message = "Error: User data file does not exist.";
      break;
    case 3:
      message = "Error: User data file not found.";
      break;
    case 4:
      message = "Error: Failed to write data to file.";
      break;
    default:
      message = "An unknown error occurred.";
  }
  console.log("Remove Data Result Message: ", message);
  return false;  
}

async function modifyData(username, service, account, password) {
//  const invoke = getInvoke();
  const resultCode = await invoke("modify_data", { username, service, account, password });
  let message = "";
  switch (resultCode) {
    case 0:
          message = "Data modified successfully.";
          return true;
        case 2:
          message = "Error: User data file does not exist.";
          break;
        case 3:
          message = "Error: User data file not found.";
          break;
        case 4:
          message = "Error: Failed to write data to file.";
          break;
        default:
          message = "An unknown error occurred.";
      }
      console.log("Modify Data Result Message: ", message);
      return false;
}

function get_current_user() {
  return getCurrentUser();
}
