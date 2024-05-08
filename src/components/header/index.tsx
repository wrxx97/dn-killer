import ButtonGroup from "@mui/material/ButtonGroup";
import IconButton from "@mui/material/IconButton";
import AddAlarmIcon from "@mui/icons-material/AddAlarm";
import SettingsIcon from "@mui/icons-material/Settings";
import CloseIcon from "@mui/icons-material/Close";
import MinimizeIcon from "@mui/icons-material/Minimize";
import HomeIcon from "@mui/icons-material/Home";
import { useLocation, useNavigate } from "react-router-dom";
import { appWindow } from "@tauri-apps/api/window";

import useStore from "@/stores";
import { ReactElement } from "react";

export default ({ left = <div /> }: { left?: ReactElement }) => {
  const store = useStore();
  const location = useLocation();
  const navigate = useNavigate();

  const isSetting = location.pathname === "/setting";

  const handleNavigate = () => {
    if (isSetting) {
      navigate("/home");
    } else {
      navigate("/setting");
    }
  };

  return (
    <div
      id="drag-header"
      data-tauri-drag-region
      style={{
        cursor: "move",
        width: "100%",
        display: "flex",
        justifyContent: "space-between",
      }}
    >
      {left}
      <ButtonGroup>
        <IconButton onClick={() => store.addTask()}>
          <AddAlarmIcon color="info" />
        </IconButton>
        <IconButton onClick={handleNavigate}>
          {isSetting ? (
            <HomeIcon color="info" />
          ) : (
            <SettingsIcon color="info" />
          )}
        </IconButton>
        <IconButton onClick={() => appWindow.minimize()}>
          <MinimizeIcon color="info" />
        </IconButton>
        <IconButton onClick={() => appWindow.close()}>
          <CloseIcon color="info" />
        </IconButton>
      </ButtonGroup>
    </div>
  );
};
