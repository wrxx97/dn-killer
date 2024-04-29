import { useMemo } from "react";
import { styled } from "@mui/material/styles";
import Box from "@mui/material/Box";
import SpeedDial, { SpeedDialProps } from "@mui/material/SpeedDial";
import SpeedDialIcon from "@mui/material/SpeedDialIcon";
import SpeedDialAction from "@mui/material/SpeedDialAction";
import FileCopyIcon from "@mui/icons-material/FileCopyOutlined";
import SaveIcon from "@mui/icons-material/Save";
import PrintIcon from "@mui/icons-material/Print";
import ShareIcon from "@mui/icons-material/Share";
import LockIcon from "@mui/icons-material/Lock";
import LockOpenIcon from "@mui/icons-material/LockOpen";
import { appWindow, LogicalSize, getCurrent } from "@tauri-apps/api/window";
import useStore from "@/stores/index";

const StyledSpeedDial = styled(SpeedDial)(({ theme }) => ({
  position: "absolute",
  "&.MuiSpeedDial-directionUp, &.MuiSpeedDial-directionLeft": {
    bottom: theme.spacing(2),
    right: theme.spacing(2),
  },
  "&.MuiSpeedDial-directionDown, &.MuiSpeedDial-directionRight": {
    top: theme.spacing(2),
    left: theme.spacing(2),
  },
}));

export default function ActionBar() {
  const store = useStore();
  const handleAddTask = async () => {
    store.updateLock();
  };
  const actions = useMemo(() => {
    return [
      {
        icon: <LockIcon />,
        name: "Lock",
        onClick: handleAddTask,
        open: !store.alwaysOnTop,
      },
      {
        icon: <LockOpenIcon />,
        name: "Unlock",
        onClick: handleAddTask,
        open: store.alwaysOnTop,
      },
      { icon: <SaveIcon />, name: "Save" },
      { icon: <PrintIcon />, name: "Print" },
      { icon: <ShareIcon />, name: "Share" },
    ];
  }, [store.alwaysOnTop]);
  return (
    <Box sx={{ position: "absolute", right: 0, bottom: 2 }}>
      <StyledSpeedDial
        ariaLabel="menu"
        icon={<SpeedDialIcon />}
        direction={"left" as SpeedDialProps["direction"]}
        className="drag-bar"
      >
        {actions
          .filter((action) => action.open !== false)
          .map((action) => (
            <SpeedDialAction
              key={action.name}
              icon={action.icon}
              tooltipTitle={action.name}
              onClick={action.onClick}
              open={action.open}
            />
          ))}
      </StyledSpeedDial>
    </Box>
  );
}
