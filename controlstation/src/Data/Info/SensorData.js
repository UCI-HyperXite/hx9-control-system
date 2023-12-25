import React, { createContext, useContext, useState } from "react";
const SharedStateContext = createContext();
export const SharedStateProvider = ({ children }) => {
  const [powersInfo, setPowersInfo] = useState([
    {
      name: "VMinus",
      value: 0,
    },
    {
      name: "VPlus",
      value: 0,
    },
    {
      name: "BMS HSV",
      value: 0,
    },
    {
      name: "Shunt Current",
      value: 10,
    },
  ]);
  return (
    <SharedStateContext.Provider value={{ powersInfo, setPowersInfo }}>
      {children}
    </SharedStateContext.Provider>
  );
};
export const useSharedState = () => {
  return useContext(SharedStateContext);
};
