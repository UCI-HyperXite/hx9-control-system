import Card from "./Card";
import { useSharedState } from "../../Data/Info/SensorData";
export default function Powers() {
  const { powersInfo } = useSharedState();
  return powersInfo.map((prop) => {
    return <Card prop={prop} />;
  });
}
