import { useRef, useState, memo } from 'react';
import { motion, useTransform, useSpring } from 'framer-motion';
import { useScrollVelocity } from '../hooks/useScrollVelocity';
import { springSmooth } from '../lib/motion';

interface TiltCardProps {
  children: React.ReactNode;
  className?: string;
}

function TiltCard({ children, className = '' }: TiltCardProps) {
  const ref = useRef<HTMLDivElement>(null);
  const [rotateX, setRotateX] = useState(0);
  const [rotateY, setRotateY] = useState(0);
  const [glareX, setGlareX] = useState(50);
  const [glareY, setGlareY] = useState(50);
  const velocity = useScrollVelocity();

  const skewXRaw = useTransform(velocity, (v) => Math.max(-2, Math.min(2, v * 8)));
  const skewX = useSpring(skewXRaw, springSmooth);

  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!ref.current) return;
    const rect = ref.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;

    setRotateX(((y - centerY) / centerY) * -6);
    setRotateY(((x - centerX) / centerX) * 6);
    setGlareX((x / rect.width) * 100);
    setGlareY((y / rect.height) * 100);
  };

  const handleMouseLeave = () => {
    setRotateX(0);
    setRotateY(0);
    setGlareX(50);
    setGlareY(50);
  };

  return (
    <motion.div
      ref={ref}
      className={`relative group ${className}`}
      onMouseMove={handleMouseMove}
      onMouseLeave={handleMouseLeave}
      animate={{
        rotateX,
        rotateY,
      }}
      transition={{ type: 'spring', stiffness: 400, damping: 25 }}
      style={{
        skewX,
        transformStyle: 'preserve-3d',
        perspective: 1000,
        willChange: 'transform',
      }}
    >
      {children}
      <div
        className="absolute inset-0 rounded-2xl pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-500"
        style={{
          background: `radial-gradient(circle at ${glareX}% ${glareY}%, rgba(255,255,255,0.08) 0%, transparent 50%)`,
        }}
      />
    </motion.div>
  );
}

export default memo(TiltCard);
