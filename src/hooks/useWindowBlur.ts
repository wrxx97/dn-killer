import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import useStore from "@/stores";

export default () => {
  const navigate = useNavigate();
  const setFocusTask = useStore((state) => state.setFocusTask);

  useEffect(() => {
    const isDev = import.meta.env.MODE === "development";
    window.onblur = isDev
      ? null
      : () => {
          navigate("/");
          setFocusTask(null);
        };
  }, []);
};
