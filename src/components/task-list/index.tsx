import Box from "@mui/material/Box";
import Accordion from "@mui/material/Accordion";
import AccordionSummary from "@mui/material/AccordionSummary";
import AccordionDetails from "@mui/material/AccordionDetails";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import LinearProgress from "@mui/material/LinearProgress";
import useInterval from "ahooks/lib/useInterval";

import Form from "@/components/task-list/Form";
import useStore from "@/stores";

export default function TaskList() {
  const store = useStore();
  useInterval(() => {}, 1000);

  return (
    <Box
      sx={{
        height: "calc(100% - 50px)",
        overflowY: "scroll",
      }}
    >
      {store.tasks.map((task) => (
        <Accordion
          defaultExpanded={true}
          expanded={store.hover}
          key={task.id}
          sx={
            store.hover
              ? {}
              : {
                  backgroundColor: "rgba(0, 0, 0, 0.6)",
                  color: "white",
                }
          }
        >
          <AccordionSummary
            expandIcon={<ExpandMoreIcon />}
            aria-controls="panel1-content"
            id="panel1-header"
          >
            <Box sx={{ width: "100%" }}>
              {`${task.title} （${180}秒）(快捷键：${task.hotkey})`}
              <LinearProgress variant="determinate" value={100} />
            </Box>
          </AccordionSummary>
          <AccordionDetails>
            <Form data={task} />
          </AccordionDetails>
        </Accordion>
      ))}
    </Box>
  );
}
