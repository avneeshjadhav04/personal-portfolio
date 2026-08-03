import { useEffect, useRef, useCallback } from 'react';
import type { ReactNode } from 'react';
import Lenis from 'lenis';
import { SmoothScrollContext } from './SmoothScrollContext';

export default function SmoothScrollProvider({ children }: { children: ReactNode }) {
  const lenisRef = useRef<Lenis | null>(null);
  const startLoopRef = useRef<() => void>(() => {});

  useEffect(() => {
    const isMobile = window.matchMedia('(max-width: 768px)').matches ||
      'ontouchstart' in window || navigator.maxTouchPoints > 0;

    const lenis = new Lenis({
      duration: isMobile ? 1.0 : 1.4,
      easing: (t: number) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
      orientation: 'vertical',
      gestureOrientation: 'vertical',
      smoothWheel: true,
      touchMultiplier: 1.5,
      infinite: false,
    });

    lenisRef.current = lenis;

    let frameId = 0;
    const tick = (time: number) => {
      frameId = 0;
      lenis.raf(time);
      // Keep the loop alive only while Lenis is actively animating a scroll.
      if (lenis.isScrolling || lenis.velocity !== 0) {
        frameId = requestAnimationFrame(tick);
      }
    };

    const start = () => {
      if (!frameId) frameId = requestAnimationFrame(tick);
    };
    startLoopRef.current = start;

    const startEvents: Array<keyof WindowEventMap> = [
      'wheel',
      'touchstart',
      'touchmove',
      'keydown',
      'scroll',
    ];
    startEvents.forEach((evt) => window.addEventListener(evt, start, { passive: true }));

    // Kick the first frame; the loop then rests until a scroll begins.
    start();

    return () => {
      if (frameId) cancelAnimationFrame(frameId);
      startEvents.forEach((evt) => window.removeEventListener(evt, start));
      lenis.destroy();
      lenisRef.current = null;
    };
  }, []);

  const scrollTo = useCallback((target: string | number | HTMLElement) => {
    startLoopRef.current();
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