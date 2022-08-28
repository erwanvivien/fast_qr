import init, { initSync, qr } from "./fast_qr.js";

export { initSync, qr };
export default init;

const shapes = [
  "Square",
  "Circle",
  "RoundedSquare",
  "Vertical",
  "Horizontal",
  "Diamond",
];
// as const;

const DEFAULT_OPTIONS = {
  background_color: "#000",
  margin: 4,
  module_color: "#FFF",
  shape: "Square",
};

// const updateOpt = (o?: Partial<QrSvgOptions>) => {
const updateOpt = (o) => {
  // const opt: QrSvgOptions = {
  const opt = {
    ...DEFAULT_OPTIONS,
  };

  if (!o) return opt;

  if (o.module_color && typeof o.module_color === "string") {
    opt.module_color = o.module_color;
  }
  if (o.margin && typeof o.margin === "number" && o.margin >= 0) {
    opt.margin = o.margin;
  }
  if (o.background_color && typeof o.background_color === "string") {
    opt.background_color = o.background_color;
  }
  if (o.shape && typeof o.shape === "string" && shapes.includes(o.shape)) {
    opt.shape = o.shape;
  }

  return opt;
};

const fmtShape =
  // : {
  //   [K in Shape]: (p: { x: number; y: number }, margin: number) => string;
  // }
  {
    Circle: (p, margin) =>
      `M${p.x + margin + 1},${p.x + margin + 0.5}a.5,.5 0 1,1 0,-.1`,
    Diamond: (p, margin) =>
      `M${p.x + margin}.5,${p.y + margin}l.5,.5l-.5,.5l-.5,-.5z`,
    Horizontal: (p, margin) => `M${p.x + margin}.1,${p.y + margin}h1v.8h-1`,
    RoundedSquare: (p, margin) =>
      `M${p.x + margin}.2,${p.y + margin}.2 ${p.x + margin}.8,${
        p.y + margin
      }.2` +
      ` ${p.x + margin}.8,${p.y + margin}.8 ${p.x + margin}.2,${
        p.y + margin
      }.8z`,
    Square: (p, margin) => `M${p.x + margin},${p.y + margin}h1v1h-1`,
    Vertical: (p, margin) => `M${p.x + margin},${p.y + margin}.1h.8v1h-.8`,
  };

// const qr_svg = (content: string, options?: Partial<QrSvgOptions>) => {
function qr_svg(content = "", options = { ...DEFAULT_OPTIONS }) {
  if (typeof content !== "string") {
    return "";
  }

  const opt = updateOpt(options);

  const QR = qr(content);
  const QRsize = Math.sqrt(QR.length);

  // const pos: { x: number, y: number }[] = [];
  const pos = [];
  for (let i = 0; i < QRsize; i += 1) {
    for (let j = 0; j < QRsize; j += 1) {
      if (QR[i * QRsize + j] !== 1) continue;

      pos.push({
        x: j,
        y: i,
      });
    }
  }

  const { background_color, margin, module_color, shape } = opt;

  /// M places the cursor and h1v1h-1 draws a square
  const svgPath = pos.map((p) => fmtShape[shape](p, margin)).join("");

  const size = QRsize + margin * 2;
  const svg = [
    `<svg
  viewBox='0 0 ${size} ${size}'
  xmlns="http://www.w3.org/2000/svg"
  xmlns:xlink="http://www.w3.org/1999/xlink">`,
    `<path d='M0,0h${size}v${size}h-${size}' fill='${background_color}' />`,
    `<path d='${svgPath}' fill='${module_color}' opacity='1' />`,
    `</svg>`,
  ];

  return svg.join("");
}

export { qr_svg };
