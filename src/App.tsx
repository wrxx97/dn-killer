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
import { appWindow, LogicalSize, getCurrent } from '@tauri-apps/api/window';
import useAddDrag from './hooks/useAddDrag';

import "./App.css";

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

export default function App() {
  const handleAddTask = async () => {
    // debugger;
    // console.log("add task");
    const size = await getCurrent();
    // debugger;
    // console.log(size);
    // await appWindow.setSize(new LogicalSize(600, 500));
  };
  useAddDrag();
  const actions = useMemo(() => {
    return [
      { icon: <FileCopyIcon />, name: "Copy", onClick: handleAddTask },
      { icon: <SaveIcon />, name: "Save" },
      { icon: <PrintIcon />, name: "Print" },
      { icon: <ShareIcon />, name: "Share" },
    ];
  }, []);
  return (
    <Box
      sx={{
        transition: "all 0.2s",
        // height: '100%',
        "&:hover,&:active,&:focus": {
          // Add hover effect
          transform: "translateZ(0px)",
          flexGrow: 1,
          background: "rgba(0, 0, 0, 0.5)",
          backdropFilter: "blur(10px)",
          borderRadius: "20px",
          boxShadow: "0 0 10px rgba(0, 0, 0, 0.1)",
        },
      }}
    >
      <Box sx={{ position: "relative", mt: 3, height: 'calc(100vh - 4px)' }}>
        <StyledSpeedDial
          ariaLabel="menu"
          icon={<SpeedDialIcon />}
          direction={"left" as SpeedDialProps["direction"]}
        >
          {actions.map((action) => (
            <SpeedDialAction
              key={action.name}
              icon={action.icon}
              tooltipTitle={action.name}
              onClick={action.onClick}
            />
          ))}
        </StyledSpeedDial>
      </Box>
    </Box>
  );
}
