import "@fontsource/fira-sans";
import "@iconify/iconify";
import "./style.css";

import Alpine from "alpinejs";
import log from "loglevel";

const init = () => {
  const doUpdate = () => {
    log.debug("updating...");

    window.Alpine = Alpine;
    window.Alpine.start();

    // Check that service workers are supported
    if ("serviceWorker" in window.navigator) {
      window.navigator.serviceWorker
        .register("/service-worker.js")
        .catch((err) => log.error("error registering service worker:", err));
    }
  };

  document.addEventListener("alpine:init", () => {
    // log.debug("alpine:init!");
    Alpine.data("clipboard", () => ({
      info: false,
      error: false,
      success: false,
      warn: false,
      snackbarText: null,
      copy(text) {
        window.navigator.clipboard
          .writeText(text)
          .then(() => {
            log.info("copied to clipboard!");
            this.toggleInfo("Copied To Clipboard!", 3000);
          })
          .catch((error) => {
            log.error("error coping to clipboard!", error);
            this.toggleError(error);
          });
      },
      toggleInfo(text, timeout = 3000) {
        setTimeout(() => (this.info = false), timeout);
        this.snackbarText = text;
        this.info = true;
      },
      toggleError(text, timeout = 3000) {
        setTimeout(() => (this.error = false), timeout);
        this.snackbarText = text;
        this.error = true;
      },
      toggleSuccess(text, timeout = 3000) {
        setTimeout(() => (this.success = false), timeout);
        this.snackbarText = text;
        this.success = true;
      },
      toggleWarn(text, timeout = 3000) {
        setTimeout(() => (this.warn = false), timeout);
        this.snackbarText = text;
        this.warn = true;
      },
    }));
  });
  document.addEventListener("alpine:initialized", () => {
    log.debug("alpine:initialized!");
  });
  doUpdate();
};

if (document.readyState !== "loading") {
  init();
} else {
  document.addEventListener("DOMContentLoaded", () => {
    init();
  });
}
