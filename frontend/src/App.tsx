import { Routes, Route } from "react-router-dom";
import "./App.css";
import Login from "./components/Login";
import CreateProject from "./components/CreateProject";
import TopNavBar from "./components/TopNavBar";
import { useInitWallet } from "./near-utils/Wallet";
function App() {
  useInitWallet({ createAccessKeyFor: "", networkId: "testnet" });
  return (
    <>
      <Routes>
        <Route path="/login" element={<Login />}></Route>
        <Route path="/create-project" element={<CreateProject />}></Route>
      </Routes>
      <TopNavBar />
    </>
  );
}

export default App;
