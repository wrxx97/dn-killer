import Backdrop from "@mui/material/Backdrop";
import Button from "@mui/material/Button";
import CircularProgress from "@mui/material/CircularProgress";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import useMount from "ahooks/lib/useMount";
import { relaunch } from "@tauri-apps/api/process";
import useStore from "@/stores";
import { useNavigate } from "react-router-dom";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogContentText from "@mui/material/DialogContentText";
import DialogTitle from "@mui/material/DialogTitle";
import { useState } from "react";
import useResizeWindow from "@/hooks/useResizeWindow";

export default () => {
  const checkUpdateFlag = useStore((store) => store.checkUpdate);
  const setCheckUpdate = useStore((store) => store.setCheckUpdate);
  const setting = useStore((store) => store.setting);
  const navigate = useNavigate();
  const [updating, setUpdating] = useState(false);

  useResizeWindow(); // 设置app的窗口大小

  useMount(() => {
    if (!setting.autoUpdate) {
      navigate("/home");
      return;
    }
    const fn = async () => {
      try {
        const { shouldUpdate } = await checkUpdate();
        if (!shouldUpdate) {
          navigate("/home");
        }
        setCheckUpdate(true);
      } catch (error) {
        console.error(error);
      }
    };
    if (!checkUpdateFlag) {
      fn();
    }
  });

  const handleUpdate = async () => {
    setUpdating(true);
    await installUpdate();
    await relaunch();
    setUpdating(false);
  };

  return updating || !checkUpdateFlag ? (
    <Backdrop
      sx={{ color: "#fff", zIndex: (theme) => theme.zIndex.drawer + 100 }}
      open={true}
    >
      <CircularProgress color="inherit" />
    </Backdrop>
  ) : (
    <Dialog
      open
      aria-labelledby="alert-dialog-title"
      aria-describedby="alert-dialog-description"
      sx={{
        width: "100vw",
        height: "100vh",
        "& .MuiDialog-paper": {
          width: "100vw",
          height: "100vh",
          maxHeight: "100vh",
          margin: 0,
        },
      }}
    >
      <DialogTitle id="alert-dialog-title">检测到有版本更新</DialogTitle>
      <DialogContent>
        <DialogContentText id="alert-dialog-slide-description">
          请点击下方按钮下载新版本
        </DialogContentText>
      </DialogContent>
      <DialogActions>
        <Button onClick={handleUpdate}>下载</Button>
        <Button onClick={() => navigate("/home")}>取消</Button>
      </DialogActions>
    </Dialog>
  );
};
