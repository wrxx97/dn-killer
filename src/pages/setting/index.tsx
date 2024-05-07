import Header from "@components/header";
import useResizeWindow from "@/hooks/useResizeWindow";
import Form from "./Form";
import useWindowBlur from "@/hooks/useWindowBlur";

export default function Setting() {
  useResizeWindow(); // 设置app的窗口大小
  useWindowBlur(); // 窗口失去焦点
  return (
    <div>
      <Header />
      <Form />
    </div>
  );
}
