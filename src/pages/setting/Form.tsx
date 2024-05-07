import { useEffect } from "react";
import Box from "@mui/material/Box";
import { useForm } from "react-hook-form";
import FormSwitch from "@/components/form-switch";
import FormTextField from "@/components/form-text-field";
import FormSlider from "@/components/form-slider";

import useStore, { Setting } from "@/stores";

export default () => {
  const setting = useStore((store) => store.setting);
  const updateSetting = useStore((store) => store.updateSetting);
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
      <FormSwitch name="notifyFlag" control={control} label="语音提示" />
      <FormTextField name="notifyTemplate" control={control} label="提示模板" />
      <FormTextField
        name="notifyDr"
        control={control}
        type="number"
        label="提醒提前时间"
      />
      <FormSlider name="opciaty" control={control} label="界面透明度" />
    </Box>
  );
};
