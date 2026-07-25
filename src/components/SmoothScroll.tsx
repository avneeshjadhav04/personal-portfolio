import { useEffect, useRef, useCallback } from 'react';
import type { ReactNode } from 'react';
import Lenis from 'lenis';
import { SmoothScrollContext } from './SmoothScrollContext';

export default function SmoothScrollProvider({ children }: { children: ReactNode }) {
  const lenisRef = useRef<Lenis | null>(null);

  useEffect(() => {
    const isMobile = window.matchMedia('(max-width: 768px)').matches ||
      'ontouchstart' in window || navigator.maxTouchPoints > 0;

    const lenis = new Lenis({
      duration: isMobile ? 1.0 : 1.2,
      easing: (t: number) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
      orientation: 'vertical',
      gestureOrientation: 'vertical',
      smoothWheel: true,
      touchMultiplier: 1.5,
      infinite: false,
    });

    lenisRef.current = lenis;

    let frameId: number;
    const rafCallback = (time: number) => {
      lenis.raf(time);
      frameId = requestAnimationFrame(rafCallback);
    };
    frameId = requestAnimationFrame(rafCallback);

    return () => {
      cancelAnimationFrame(frameId);
      lenis.destroy();
    };
  }, []);

  const scrollTo = useCallback((target: string | number | HTMLElement) => {
    lenisRef.current?.scrollTo(target as string | number, {
      offset: -80,
      duration: 1.2,
    });
  }, []);

  return (
    <SmoothScrollContext.Provider value={{ scrollTo }}>
      {children}
    </SmoothScrollContext.Provider>
  );
}