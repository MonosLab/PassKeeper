// When using the Tauri API npm package:
// Define invoke in bundle.js as shown below
// window.tauriInvoke

function setCurrentUser(user) {
  sessionStorage.setItem("currentUser", JSON.stringify(user));
}

function getCurrentUser() {
  const raw = sessionStorage.getItem("currentUser");
  return raw ? JSON.parse(raw) : null;
}

async function createAccount(username, password) {
  const resultCode = await window.tauriInvoke("save_account", { username, password });
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
  const isValid = await window.tauriInvoke("validate_login", { username, password });
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
  const data = await window.tauriInvoke("get_data", { username });
  return data;
}

async function saveData(username, service, account, password) {
  const resultCode = await window.tauriInvoke("save_data", { username, service, account, password });
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
  const resultCode = await window.tauriInvoke("remove_data", { username, service });
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
  const resultCode = await window.tauriInvoke("modify_data", { username, service, account, password });
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

// Expose globally
window.login = login;
window.getData = getData;
window.saveData = saveData;
window.removeData = removeData;
window.modifyData = modifyData;
window.createAccount = createAccount;
window.setCurrentUser = setCurrentUser;
window.getCurrentUser = getCurrentUser;
