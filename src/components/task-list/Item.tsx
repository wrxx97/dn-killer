import Box from "@mui/material/Box";
import Accordion from "@mui/material/Accordion";
import AccordionSummary from "@mui/material/AccordionSummary";
import AccordionDetails from "@mui/material/AccordionDetails";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import LinearProgress from "@mui/material/LinearProgress";

import Form from "@/components/task-list/Form";
import useStore, { getLeftDuration, processTask, Task } from "@/stores";

export default ({ task }: { task: Task }) => {
  const focusTask = useStore((store) => store.focusTask);
  const setFocusTask = useStore((store) => store.setFocusTask);
  const setting = useStore((store) => store.setting);
  const isFocus = focusTask?.id === task.id;

  const getColor = () => {
    const progress = processTask(task);
    if (progress > 50) {
      return "primary"; // 蓝色 (默认颜色)
    } else if (progress > 20) {
      return "warning"; // 黄色
    } else {
      return "error"; // 红色
    }
  };

  return (
    <Accordion
      defaultExpanded={false}
      expanded={isFocus}
      onChange={() => setFocusTask(isFocus ? null : task)}
      key={task.id}
      sx={
        isFocus
          ? {
              "&.Mui-expanded": {
                margin: 0,
              },
              "&.MuiAccordion-root": {
                boxShadow: "none",
                background: "rgba(0, 0, 0, 0.6)",
              },
            }
          : {
              "&.MuiAccordion-root": {
                boxShadow: "none",
                background: "transparent",
                color: setting.fontColor,
              },
              "&.MuiAccordion-root:before": {
                display: "none",
              },
            }
      }
    >
      <AccordionSummary expandIcon={<ExpandMoreIcon fontSize="large" />}>
        <Box sx={{ width: "100%" }}>
          {task.type !== "exit" ? (
            <>
              {`${task.title} （${getLeftDuration(task)}秒）(快捷键：${
                task.hotkey
              })`}
              <LinearProgress
                variant="determinate"
                value={processTask(task)}
                color={getColor()}
              />
            </>
          ) : (
            `退出 ${task.title} (快捷键：${task.hotkey})`
          )}
        </Box>
      </AccordionSummary>
      <AccordionDetails>
        <Form data={task} />
      </AccordionDetails>
    </Accordion>
  );
};
