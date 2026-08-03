interface SectionGlowProps {
  color: string;
  position?: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'center';
  size?: 'sm' | 'md' | 'lg' | 'xl';
  opacity?: number;
  animate?: boolean;
}

const sizeMap = {
  sm: 'w-[25vw] h-[25vw] min-w-[200px] min-h-[200px]',
  md: 'w-[35vw] h-[35vw] min-w-[280px] min-h-[280px]',
  lg: 'w-[45vw] h-[45vw] min-w-[360px] min-h-[360px]',
  xl: 'w-[55vw] h-[55vw] min-w-[440px] min-h-[440px]',
};

const positionMap = {
  'top-left': '-top-[10vw] -left-[10vw]',
  'top-right': '-top-[10vw] -right-[10vw]',
  'bottom-left': '-bottom-[10vw] -left-[10vw]',
  'bottom-right': '-bottom-[10vw] -right-[10vw]',
  center: 'top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2',
};

export default function SectionGlow({
  color,
  position = 'top-right',
  size = 'lg',
  opacity = 0.35,
  animate = true,
}: SectionGlowProps) {
  return (
    <div
      className={`absolute ${positionMap[position]} ${sizeMap[size]} rounded-full pointer-events-none -z-10 ${animate ? 'animate-float' : ''}`}
      style={{
        background: `radial-gradient(circle, ${color} 0%, transparent 70%)`,
        opacity,
      }}
      aria-hidden="true"
    />
  );
}
