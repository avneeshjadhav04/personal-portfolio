import { useEffect, useRef } from 'react';
import { useMotionValue } from 'motion/react';
import type { MotionValue } from 'motion/react';

export function useScrollVelocity(): MotionValue<number> {
  const velocity = useMotionValue(0);

  useEffect(() => {
    let lastScrollY = window.scrollY;
    let lastTime = performance.now();
    let rafId = 0;

    const handleScroll = () => {
      if (rafId) return;
      rafId = requestAnimationFrame(() => {
        const now = performance.now();
        const currentScrollY = window.scrollY;
        const deltaY = currentScrollY - lastScrollY;
        const deltaTime = now - lastTime;

        if (deltaTime > 0) {
          velocity.set(deltaY / deltaTime);
        }

        lastScrollY = currentScrollY;
        lastTime = now;
        rafId = 0;
      });
    };

    window.addEventListener('scroll', handleScroll, { passive: true });
    return () => {
      window.removeEventListener('scroll', handleScroll);
      if (rafId) cancelAnimationFrame(rafId);
    };
  }, [velocity]);

  return velocity;
}

/* Throttled scroll handler for non-animated logic */
export function useThrottledScroll(callback: () => void, delay: number = 100) {
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    const handleScroll = () => {
      if (timeoutRef.current) return;
      timeoutRef.current = setTimeout(() => {
        callback();
        timeoutRef.current = null;
      }, delay);
    };

    window.addEventListener('scroll', handleScroll, { passive: true });
    return () => {
      window.removeEventListener('scroll', handleScroll);
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, [callback, delay]);
}