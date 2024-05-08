import { useEffect } from "react";
import Box from "@mui/material/Box";
import { useForm } from "react-hook-form";
import FormSwitch from "@/components/form-switch";
import FormTextField from "@/components/form-text-field";
import FormSlider from "@/components/form-slider";
import FormColorPicker from "@/components/form-color-picker";
import Grid from "@mui/material/Grid";
import IconButton from "@mui/material/IconButton";
import RefreshIcon from "@mui/icons-material/Refresh";
import { useNavigate } from "react-router-dom";
import { checkUpdate } from "@tauri-apps/api/updater";

import useStore, { Setting } from "@/stores";

export default () => {
  const setting = useStore((store) => store.setting);
  const updateSetting = useStore((store) => store.updateSetting);
  const checkUpdateFlag = useStore((store) => store.checkUpdate);
  const navigate = useNavigate();
  const setCheckUpdate = useStore((store) => store.setCheckUpdate);
  const save = useStore((store) => store.save);
  const { watch, control, setFocus } = useForm({
    defaultValues: setting,
    values: setting,
  });

  useEffect(() => {
    const subscription = watch((value) => {
      const data = value as Setting;
      updateSetting({ ...data });
      save();
    });
    return () => subscription.unsubscribe();
  }, [watch]);

  useEffect(() => {
    setFocus("notifyTemplate");
  }, [setFocus]);

  const handleCheckUpdate = async () => {
    setCheckUpdate(true);
    try {
      const { shouldUpdate } = await checkUpdate();
      if (shouldUpdate) {
        navigate("/");
      }
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <Box
      component="form"
      sx={{
        display: "flex",
        alignItems: "flex-start",
        flexDirection: "column",
        gap: "1rem",
        color: "white",
        padding: "1rem",
      }}
      noValidate
      autoComplete="true"
    >
      <Grid container spacing={2}>
        <Grid item xs={6}>
          <FormSwitch
            name="autoUpdate"
            control={control}
            label="自动检查更新"
          />
        </Grid>
        <Grid item xs={6}>
          检查更新
          <IconButton onClick={handleCheckUpdate} sx={{ marginLeft: 2 }}>
            <RefreshIcon />
          </IconButton>
        </Grid>
        <Grid item xs={6}>
          <FormSwitch name="notifyFlag" control={control} label="语音提示" />
        </Grid>
        <Grid item xs={2}>
          界面透明度
        </Grid>
        <Grid item xs={4}>
          <FormSlider name="opciaty" control={control} label="界面透明度" />
        </Grid>
        <Grid item xs={12}>
          <FormColorPicker
            name="fontColor"
            control={control}
            label="文本颜色"
          />
        </Grid>
        <Grid item xs={12}>
          <FormTextField
            name="notifyTemplate"
            control={control}
            label="提示模板"
          />
        </Grid>
        <Grid item xs={12}>
          <FormTextField
            name="notifyDr"
            control={control}
            type="number"
            label="提醒提前时间"
          />
        </Grid>
      </Grid>
    </Box>
  );
};
