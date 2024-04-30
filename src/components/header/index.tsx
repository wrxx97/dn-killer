import ButtonGroup from "@mui/material/ButtonGroup";
import IconButton from "@mui/material/IconButton";
import AddAlarmIcon from "@mui/icons-material/AddAlarm";
import SettingsIcon from "@mui/icons-material/Settings";
import CloseIcon from '@mui/icons-material/Close';
import MinimizeIcon from '@mui/icons-material/Minimize';

import { appWindow } from "@tauri-apps/api/window";

import useStore from "@/stores";

export default () => {
  const store = useStore();
  return (
    <div
      data-tauri-drag-region
      style={{
        cursor: "move",
        width: "100%",
        textAlign: "right",
      }}
    >
      <ButtonGroup>
        <IconButton onClick={() => store.addTask()}>
          <AddAlarmIcon color="info" />
        </IconButton>
        <IconButton>
          <SettingsIcon color="info" />
        </IconButton>
        <IconButton onClick={() => appWindow.minimize()}>
          <MinimizeIcon color="info" />
        </IconButton>
        <IconButton onClick={() => appWindow.close()} >
          <CloseIcon color="info" />
        </IconButton>
      </ButtonGroup>
    </div>
  );
};
