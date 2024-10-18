import { Route, Routes } from "react-router-dom";

import IndexPage from "@/pages/index";
import ProgrammerPage from "./pages/programmer";
import IOPage from "./pages/io";
import ControllerPage from "./pages/controller";
import TwinPage from "./pages/twin";

function App() {
  return (
    <Routes>
      <Route element={<IndexPage />} path="/" />
      <Route element={<ProgrammerPage />} path="/programmer" />
      <Route element={<IOPage />} path="/io" />
      <Route element={<ControllerPage />} path="/controller" />
      <Route element={<TwinPage />} path="/twin" />
    </Routes>
  );
}

export default App;
