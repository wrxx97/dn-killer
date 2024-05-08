import Box from "@mui/material/Box";
import Modal from "@mui/material/Modal";
import Button from "@mui/material/Button";
import CircularProgress from "@mui/material/CircularProgress";
import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
} from "@tauri-apps/api/updater";
import useMount from "ahooks/lib/useMount";
import { relaunch } from "@tauri-apps/api/process";
import useStore from "@/stores";
import { useNavigate } from "react-router-dom";

export default () => {
  const checkUpdateFlag = useStore((store) => store.checkUpdate);
  const setCheckUpdate = useStore((store) => store.setCheckUpdate);
  const navigate = useNavigate();
  useMount(() => {
    const fn = async () => {
      setCheckUpdate(true);
      try {
        const { shouldUpdate, manifest } = await checkUpdate();
        if (!shouldUpdate) {
          navigate("/home");
        }
      } catch (error) {
        console.error(error);
      }
    };
    if (!checkUpdateFlag) {
      fn();
    }
  });

  const handleUpdate = async () => {
    await installUpdate();
    await relaunch();
  };

  return checkUpdateFlag ? (
    <Modal
      open
      aria-labelledby="child-modal-title"
      aria-describedby="child-modal-description"
    >
      <Box
        sx={{
          width: "100%",
          border: "none",
          display: "flex",
          alignItems: "center",
        }}
      >
        <h2 id="child-modal-title">检测到有版本更新</h2>
        <p id="child-modal-description">请点击下方按钮下载新版本</p>
        <Button onClick={handleUpdate}>下载</Button>
        <Button onClick={() => navigate("/home")}>取消</Button>
      </Box>
    </Modal>
  ) : (
    <CircularProgress />
  );
};
