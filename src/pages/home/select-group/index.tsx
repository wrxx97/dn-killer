import { useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import Box from "@mui/material/Box";
import Button from "@mui/material/Button";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import IconButton from "@mui/material/IconButton";
import ButtonGroup from "@mui/material/ButtonGroup";
import DeleteIcon from "@mui/icons-material/Delete";
import AddIcon from "@mui/icons-material/Add";
import FormTextField from "@/components/form-text-field";

import useStore, { Task } from "@/stores";

export default function SelectGroup() {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  const currentGroup = useStore((state) => state.currentGroup);
  const groups = useStore((state) => state.groups);
  const addGroup = useStore((state) => state.addGroup);
  const { control, watch } = useForm({
    values: {
      currentGroup,
      ...groups.reduce((pre, cur) => {
        pre[cur.id] = cur;
        return pre;
      }, {} as any),
    },
  });

  useEffect(() => {
    const subscription = watch((value) => {
      const data = value as Task;
      console.info(data);
      // isExit
      //   ? store.updateExitTask({ ...data })
      //   : store.updateTask({ ...data });
      // store.save();
    });
    return () => subscription.unsubscribe();
  }, [watch]);

  return (
    <Box component="form" noValidate autoComplete="true">
      <Controller
        name="currentGroup"
        control={control}
        render={() => (
          <>
            <Button
              id="demo-positioned-button"
              aria-controls={open ? "demo-positioned-menu" : undefined}
              aria-haspopup="true"
              aria-expanded={open ? "true" : undefined}
              onClick={handleClick}
              endIcon={<ExpandMoreIcon />}
            >
              默认分组
            </Button>
            <Menu
              id="demo-positioned-menu"
              aria-labelledby="demo-positioned-button"
              anchorEl={anchorEl}
              open={open}
              onClose={handleClose}
              anchorOrigin={{
                vertical: "top",
                horizontal: "left",
              }}
              transformOrigin={{
                vertical: "top",
                horizontal: "left",
              }}
            >
              {groups.map((group) => (
                <MenuItem key={group.id} value={group.id}>
                  <FormTextField
                    name={`${group.id}.name`}
                    control={control}
                    size="small"
                  />
                  <ButtonGroup size="small" aria-label="Small button group">
                    <IconButton onClick={() => addGroup("")} key="add">
                      <AddIcon />
                    </IconButton>
                    <IconButton
                      key="delete"
                      onClick={() => {
                        // store.deleteTask(data.id);
                        // store.save();
                      }}
                    >
                      <DeleteIcon />
                    </IconButton>
                  </ButtonGroup>
                </MenuItem>
              ))}
            </Menu>
          </>
        )}
      />
    </Box>
  );
}
