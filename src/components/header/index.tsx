import ButtonGroup from "@mui/material/ButtonGroup";
import IconButton from "@mui/material/IconButton";
import AddAlarmIcon from "@mui/icons-material/AddAlarm";
import SettingsIcon from "@mui/icons-material/Settings";

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
          <AddAlarmIcon />
        </IconButton>
        <IconButton>
          <SettingsIcon />
        </IconButton>
      </ButtonGroup>
    </div>
  );
};
