import "./style.css";

export default function Card({ prop }) {
  return (
    <div id="card" className="extra">
      <h1 id="cardHeader">{prop.name}</h1>
      <h1 id="cardinfo">{prop.value}</h1>
    </div>
  );
}
