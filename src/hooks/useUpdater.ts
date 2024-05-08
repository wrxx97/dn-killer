import { useEffect } from "react";
import {
  checkUpdate,
  // installUpdate,
  // onUpdaterEvent,
} from "@tauri-apps/api/updater";
// import { relaunch } from "@tauri-apps/api/process";

export default () => {
  useEffect(() => {
    const fn = async () => {
      try {
        const { shouldUpdate, manifest } = await checkUpdate();
        if (shouldUpdate) {
          // You could show a dialog asking the user if they want to install the update here.
          console.log(
            `Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
          );
          // await installUpdate();
          // await relaunch();
        }
      } catch (error) {
        console.error(error);
      }
    };
    fn();
  }, []);
};
