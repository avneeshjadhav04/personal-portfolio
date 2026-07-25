import type { Variants } from 'motion/react';

/**
 * Canonical easing curves. Use these instead of ad-hoc cubic-beziers
 * so motion feels consistent across the whole site.
 */
export const easeOutExpo = [0.16, 1, 0.3, 1] as const;      // primary entrances (Hero)
export const easeStandard = [0.4, 0, 0.2, 1] as const;       // UI transitions (Preloader, Navbar)
export const easeSmooth = [0.22, 1, 0.36, 1] as const;       // secondary entrances (cards, sections)
export const easeInOut = [0.65, 0, 0.35, 1] as const;        // dividers / bi-directional

/**
 * Spring configs. Critically-damped springs track input without overshoot.
 * Use for scroll-tied transforms and continuous pointer-driven motion.
 */
export const springSmooth = { stiffness: 300, damping: 40, restDelta: 0.001 };
export const springSnappy = { stiffness: 200, damping: 25, restDelta: 0.001 };

/**
 * Shared viewport config for `whileInView` — play once, trigger slightly
 * before the element is fully in view.
 */
export const viewportOnce = { once: true, margin: '-15% 0px' } as const;

/**
 * Variant factories. Keep durations short and let easing do the work.
 */
export function fadeUp(distance = 30, duration = 0.7): Variants {
  return {
    hidden: { opacity: 0, y: distance },
    show: {
      opacity: 1,
      y: 0,
      transition: { duration, ease: easeSmooth },
    },
  };
}

export function staggerContainer(stagger = 0.1, delayChildren = 0): Variants {
  return {
    hidden: {},
    show: { transition: { staggerChildren: stagger, delayChildren } },
  };
}

export function cardVariants(distance = 40, duration = 0.6): Variants {
  return {
    hidden: { opacity: 0, y: distance, scale: 0.95 },
    show: {
      opacity: 1,
      y: 0,
      scale: 1,
      transition: { duration, ease: easeSmooth },
    },
  };
}