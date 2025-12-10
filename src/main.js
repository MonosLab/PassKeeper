const { invoke } = window.__TAURI__.core;

// State
let passwords = [];
let currentEditId = null;

// Screen elements
const unlockScreen = document.getElementById("unlock-screen");
const mainScreen = document.getElementById("main-screen");

// Unlock form
const unlockForm = document.getElementById("unlock-form");
const masterPasswordInput = document.getElementById("master-password");
const unlockError = document.getElementById("unlock-error");

// Main screen elements
const searchInput = document.getElementById("search-input");
const addBtn = document.getElementById("add-btn");
const lockBtn = document.getElementById("lock-btn");
const passwordsList = document.getElementById("passwords-list");

// Modal elements
const passwordModal = document.getElementById("password-modal");
const modalTitle = document.getElementById("modal-title");
const passwordForm = document.getElementById("password-form");
const passwordIdInput = document.getElementById("password-id");
const cancelBtn = document.getElementById("cancel-btn");

// Password form inputs
const pwdTitle = document.getElementById("pwd-title");
const pwdUsername = document.getElementById("pwd-username");
const pwdPassword = document.getElementById("pwd-password");
const pwdUrl = document.getElementById("pwd-url");
const pwdNotes = document.getElementById("pwd-notes");
const togglePasswordBtn = document.getElementById("toggle-password");
const generateBtn = document.getElementById("generate-btn");

// Generator modal
const generatorModal = document.getElementById("generator-modal");
const pwdLength = document.getElementById("pwd-length");
const lengthValue = document.getElementById("length-value");
const useUppercase = document.getElementById("use-uppercase");
const useNumbers = document.getElementById("use-numbers");
const useSymbols = document.getElementById("use-symbols");
const generatedPwd = document.getElementById("generated-pwd");
const copyGenerated = document.getElementById("copy-generated");
const regenerateBtn = document.getElementById("regenerate-btn");
const useGeneratedBtn = document.getElementById("use-generated-btn");
const closeGeneratorBtn = document.getElementById("close-generator-btn");

// Initialize
window.addEventListener("DOMContentLoaded", async () => {
  checkUnlockStatus();
  setupEventListeners();
});

async function checkUnlockStatus() {
  try {
    const unlocked = await invoke("is_unlocked");
    if (unlocked) {
      showMainScreen();
    }
  } catch (error) {
    console.error("Error checking unlock status:", error);
  }
}

function setupEventListeners() {
  // Unlock form
  unlockForm.addEventListener("submit", handleUnlock);

  // Main screen
  addBtn.addEventListener("click", openAddModal);
  lockBtn.addEventListener("click", handleLock);
  searchInput.addEventListener("input", handleSearch);

  // Password modal
  cancelBtn.addEventListener("click", closeModal);
  passwordForm.addEventListener("submit", handleSavePassword);
  togglePasswordBtn.addEventListener("click", togglePasswordVisibility);
  generateBtn.addEventListener("click", openGenerator);

  // Generator modal
  pwdLength.addEventListener("input", () => {
    lengthValue.textContent = pwdLength.value;
  });
  regenerateBtn.addEventListener("click", generatePassword);
  useGeneratedBtn.addEventListener("click", useGeneratedPassword);
  closeGeneratorBtn.addEventListener("click", closeGenerator);
  copyGenerated.addEventListener("click", copyGeneratedPassword);
}

// Unlock functionality
async function handleUnlock(e) {
  e.preventDefault();
  unlockError.textContent = "";

  try {
    const masterPassword = masterPasswordInput.value;
    await invoke("unlock_storage", { masterPassword });
    showMainScreen();
  } catch (error) {
    unlockError.textContent = error || "Failed to unlock. Please try again.";
  }
}

async function handleLock() {
  try {
    await invoke("lock_storage");
    passwords = [];
    unlockScreen.classList.remove("hidden");
    mainScreen.classList.add("hidden");
    masterPasswordInput.value = "";
    unlockError.textContent = "";
  } catch (error) {
    alert("Error locking storage: " + error);
  }
}

async function showMainScreen() {
  unlockScreen.classList.add("hidden");
  mainScreen.classList.remove("hidden");
  await loadPasswords();
}

// Password management
async function loadPasswords() {
  try {
    passwords = await invoke("get_all_passwords");
    renderPasswords(passwords);
  } catch (error) {
    console.error("Error loading passwords:", error);
    alert("Failed to load passwords. Please ensure the application is unlocked and try again. Error: " + error);
  }
}

function renderPasswords(passwordsToRender) {
  if (passwordsToRender.length === 0) {
    passwordsList.innerHTML = '<p class="empty-state">No passwords found.</p>';
    return;
  }

  passwordsList.innerHTML = passwordsToRender
    .map(
      (pwd) => `
    <div class="password-card" data-id="${pwd.id}">
      <h3>${escapeHtml(pwd.title)}</h3>
      <div class="info"><strong>Username:</strong> ${escapeHtml(pwd.username)}</div>
      <div class="info"><strong>Password:</strong> ${"•".repeat(8)}</div>
      ${pwd.url ? `<div class="info"><strong>URL:</strong> ${escapeHtml(pwd.url)}</div>` : ""}
      ${pwd.notes ? `<div class="info"><strong>Notes:</strong> ${escapeHtml(pwd.notes.substring(0, 50))}${pwd.notes.length > 50 ? "..." : ""}</div>` : ""}
      <div class="actions">
        <button class="btn-secondary" onclick="viewPassword('${pwd.id}')">View</button>
        <button class="btn-primary" onclick="editPassword('${pwd.id}')">Edit</button>
        <button class="btn-danger" onclick="deletePassword('${pwd.id}')">Delete</button>
      </div>
    </div>
  `
    )
    .join("");
}

function escapeHtml(text) {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}

// Modal functionality
function openAddModal() {
  currentEditId = null;
  modalTitle.textContent = "Add Password";
  passwordForm.reset();
  passwordIdInput.value = "";
  passwordModal.classList.remove("hidden");
}

async function editPassword(id) {
  try {
    const password = await invoke("get_password", { id });
    if (!password) {
      alert("Password not found");
      return;
    }

    currentEditId = id;
    modalTitle.textContent = "Edit Password";
    passwordIdInput.value = id;
    pwdTitle.value = password.title;
    pwdUsername.value = password.username;
    pwdPassword.value = password.password;
    pwdUrl.value = password.url || "";
    pwdNotes.value = password.notes || "";
    passwordModal.classList.remove("hidden");
  } catch (error) {
    alert("Error loading password: " + error);
  }
}

async function viewPassword(id) {
  try {
    const password = await invoke("get_password", { id });
    if (!password) {
      alert("Password not found");
      return;
    }

    // Copy to clipboard without displaying in alert for better security
    await navigator.clipboard.writeText(password.password);
    alert("Password copied to clipboard!");
  } catch (error) {
    alert("Error viewing password: " + error);
  }
}

async function deletePassword(id) {
  if (!confirm("Are you sure you want to delete this password?")) {
    return;
  }

  try {
    await invoke("delete_password", { id });
    await loadPasswords();
  } catch (error) {
    alert("Error deleting password: " + error);
  }
}

function closeModal() {
  passwordModal.classList.add("hidden");
  passwordForm.reset();
  currentEditId = null;
}

async function handleSavePassword(e) {
  e.preventDefault();

  const title = pwdTitle.value;
  const username = pwdUsername.value;
  const password = pwdPassword.value;
  const url = pwdUrl.value || null;
  const notes = pwdNotes.value || null;

  try {
    if (currentEditId) {
      await invoke("update_password", {
        id: currentEditId,
        title,
        username,
        password,
        url,
        notes,
      });
    } else {
      await invoke("add_password", {
        title,
        username,
        password,
        url,
        notes,
      });
    }

    closeModal();
    await loadPasswords();
  } catch (error) {
    alert("Error saving password: " + error);
  }
}

function togglePasswordVisibility() {
  const type = pwdPassword.type === "password" ? "text" : "password";
  pwdPassword.type = type;
  togglePasswordBtn.textContent = type === "password" ? "👁️" : "🙈";
}

// Search functionality
function handleSearch(e) {
  const query = e.target.value.toLowerCase();
  if (query === "") {
    renderPasswords(passwords);
  } else {
    const filtered = passwords.filter(
      (pwd) =>
        pwd.title.toLowerCase().includes(query) ||
        pwd.username.toLowerCase().includes(query) ||
        (pwd.url && pwd.url.toLowerCase().includes(query))
    );
    renderPasswords(filtered);
  }
}

// Password generator
function openGenerator() {
  generatorModal.classList.remove("hidden");
  generatePassword();
}

function closeGenerator() {
  generatorModal.classList.add("hidden");
}

async function generatePassword() {
  try {
    const length = parseInt(pwdLength.value);
    const useSymbolsVal = useSymbols.checked;
    const useNumbersVal = useNumbers.checked;
    const useUppercaseVal = useUppercase.checked;

    const generated = await invoke("generate_password", {
      length,
      useSymbols: useSymbolsVal,
      useNumbers: useNumbersVal,
      useUppercase: useUppercaseVal,
    });

    generatedPwd.value = generated;
  } catch (error) {
    alert("Error generating password: " + error);
  }
}

function useGeneratedPassword() {
  pwdPassword.value = generatedPwd.value;
  closeGenerator();
}

async function copyGeneratedPassword() {
  try {
    await navigator.clipboard.writeText(generatedPwd.value);
    copyGenerated.textContent = "Copied!";
    setTimeout(() => {
      copyGenerated.textContent = "Copy";
    }, 2000);
  } catch (error) {
    alert("Failed to copy to clipboard");
  }
}

// Make functions available globally for onclick handlers
window.editPassword = editPassword;
window.viewPassword = viewPassword;
window.deletePassword = deletePassword;
