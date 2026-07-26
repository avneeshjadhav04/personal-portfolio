import { useRef, memo } from 'react';
import { motion, useTransform, useSpring, useMotionValue, useMotionTemplate } from 'motion/react';
import { useScrollVelocity } from '../hooks/useScrollVelocity';
import { springSmooth } from '../lib/motion';

interface TiltCardProps {
  children: React.ReactNode;
  className?: string;
}

function TiltCard({ children, className = '' }: TiltCardProps) {
  const ref = useRef<HTMLDivElement>(null);
  const rotateXRaw = useMotionValue(0);
  const rotateYRaw = useMotionValue(0);
  const glareX = useMotionValue(50);
  const glareY = useMotionValue(50);
  const velocity = useScrollVelocity();

  const rotateX = useSpring(rotateXRaw, { stiffness: 400, damping: 25 });
  const rotateY = useSpring(rotateYRaw, { stiffness: 400, damping: 25 });
  const skewXRaw = useTransform(velocity, (v) => Math.max(-2, Math.min(2, v * 8)));
  const skewX = useSpring(skewXRaw, springSmooth);
  const glareBackground = useMotionTemplate`radial-gradient(circle at ${glareX}% ${glareY}%, rgba(255,255,255,0.08) 0%, transparent 50%)`;

  const rafId = useRef(0);
  const pendingMove = useRef<{ x: number; y: number } | null>(null);

  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    pendingMove.current = { x: e.clientX, y: e.clientY };
    if (rafId.current) return;
    rafId.current = requestAnimationFrame(() => {
      rafId.current = 0;
      const move = pendingMove.current;
      if (!move || !ref.current) return;
      const rect = ref.current.getBoundingClientRect();
      const x = move.x - rect.left;
      const y = move.y - rect.top;
      const centerX = rect.width / 2;
      const centerY = rect.height / 2;

      rotateXRaw.set(((y - centerY) / centerY) * -6);
      rotateYRaw.set(((x - centerX) / centerX) * 6);
      glareX.set((x / rect.width) * 100);
      glareY.set((y / rect.height) * 100);
    });
  };

  const handleMouseLeave = () => {
    if (rafId.current) {
      cancelAnimationFrame(rafId.current);
      rafId.current = 0;
    }
    pendingMove.current = null;
    rotateXRaw.set(0);
    rotateYRaw.set(0);
    glareX.set(50);
    glareY.set(50);
  };

  return (
    <motion.div
      ref={ref}
      className={`relative group ${className}`}
      onMouseMove={handleMouseMove}
      onMouseLeave={handleMouseLeave}
      style={{
        rotateX,
        rotateY,
        skewX,
        transformStyle: 'preserve-3d',
        perspective: 1000,
        backfaceVisibility: 'hidden',
        willChange: 'transform',
      }}
    >
      {children}
      <motion.div
        className="absolute inset-0 rounded-2xl pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-500"
        style={{ background: glareBackground }}
      />
    </motion.div>
  );
}

export default memo(TiltCard);