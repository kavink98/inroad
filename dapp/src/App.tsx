import { Routes, Route } from "react-router-dom";
import "./App.css";
import Login from "./components/Login";
import CreateProject from "./components/CreateProject";
function App() {
  return (
    <Routes>
      <Route path="/login" element={<Login />}></Route>
      <Route path="/create-project" element={<CreateProject />}></Route>
    </Routes>
  );
}

export default App;
