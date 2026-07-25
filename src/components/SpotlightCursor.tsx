import { useEffect, useRef, useState } from 'react';
import { motion, useMotionValue } from 'framer-motion';

export default function SpotlightCursor() {
  const x = useMotionValue(0);
  const y = useMotionValue(0);
  const [isVisible, setIsVisible] = useState(false);
  const [isTouchDevice, setIsTouchDevice] = useState(false);
  const visibleRef = useRef(false);

  useEffect(() => {
    const checkTouch = () => {
      setIsTouchDevice(
        'ontouchstart' in window || navigator.maxTouchPoints > 0
      );
    };
    checkTouch();

    const handleMouseMove = (e: MouseEvent) => {
      x.set(e.clientX);
      y.set(e.clientY);
      if (!visibleRef.current) {
        visibleRef.current = true;
        setIsVisible(true);
      }
    };

    const handleMouseLeave = () => {
      visibleRef.current = false;
      setIsVisible(false);
    };

    window.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseleave', handleMouseLeave);

    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseleave', handleMouseLeave);
    };
  }, [x, y]);

  if (isTouchDevice) return null;

  return (
    <motion.div
      className="fixed pointer-events-none z-[5]"
      style={{
        x,
        y,
        translateX: -150,
        translateY: -150,
        width: 300,
        height: 300,
        borderRadius: '50%',
        background: 'radial-gradient(circle, rgba(20, 184, 166, 0.06) 0%, rgba(99, 102, 241, 0.03) 40%, transparent 70%)',
        willChange: 'transform, opacity',
        opacity: isVisible ? 1 : 0,
      }}
      transition={{ type: 'spring', stiffness: 150, damping: 15, mass: 0.2 }}
    />
  );
}