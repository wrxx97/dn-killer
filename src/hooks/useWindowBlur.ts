import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import useStore from "@/stores";

export default () => {
  const navigate = useNavigate();
  const setFocusTask = useStore((state) => state.setFocusTask);

  useEffect(() => {
    window.onblur = () => {
      navigate("/");
      setFocusTask(null);
    };
  }, []);
};
