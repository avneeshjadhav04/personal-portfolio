import { useEffect, useRef } from 'react';
import { motion, useMotionValue, useSpring, type Variants } from 'motion/react';
import { easeOutExpo, easeInOut, easeSmooth } from '../lib/motion';

const heroStaggerVariants: Variants = {
  hidden: { y: 150, opacity: 0, rotateZ: 2 },
  show: { y: 0, opacity: 1, rotateZ: 0, transition: { duration: 1.5, ease: easeOutExpo } },
};

const dividerVariants: Variants = {
  hidden: { scaleX: 0 },
  show: { scaleX: 1, transition: { duration: 1.2, ease: easeInOut } },
};

const subVariants: Variants = {
  hidden: { y: 30, opacity: 0 },
  show: { y: 0, opacity: 1, transition: { duration: 1, ease: easeSmooth } },
};

const ctaVariants: Variants = {
  hidden: { y: 20, opacity: 0 },
  show: { y: 0, opacity: 1, transition: { duration: 0.8, ease: easeSmooth } },
};

const ctaGroupVariants: Variants = {
  hidden: {},
  show: { transition: { staggerChildren: 0.1 } },
};

const scrollVariants: Variants = {
  hidden: { opacity: 0, y: -20 },
  show: { opacity: 1, y: 0, transition: { duration: 1, ease: easeSmooth } },
};

const heroContainer: Variants = {
  hidden: {},
  show: {
    transition: { delayChildren: 0.25, staggerChildren: 0.32 },
  },
};

export default function Hero() {
  const sectionRef = useRef<HTMLElement>(null);

  const mouseX = useMotionValue(0);
  const mouseY = useMotionValue(0);
  const smoothX = useSpring(mouseX, { stiffness: 50, damping: 20, restDelta: 0.001 });
  const smoothY = useSpring(mouseY, { stiffness: 50, damping: 20, restDelta: 0.001 });

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      const { clientX, clientY } = e;
      const centerX = window.innerWidth / 2;
      const centerY = window.innerHeight / 2;
      mouseX.set((clientX - centerX) * 0.02);
      mouseY.set((clientY - centerY) * 0.02);
    };

    window.addEventListener('mousemove', handleMouseMove);
    return () => window.removeEventListener('mousemove', handleMouseMove);
  }, [mouseX, mouseY]);

  return (
    <section
      ref={sectionRef}
      className="relative w-full min-h-[100dvh] flex items-center justify-center overflow-hidden pt-20"
    >
      <motion.div
        className="relative z-10 w-full max-w-7xl mx-auto px-6 md:px-10 flex flex-col items-center text-center"
        variants={heroContainer}
        initial="hidden"
        animate="show"
      >
        {/* Massive Typography Group */}
        <motion.div
          className="parallax-layer overflow-hidden mb-2"
          style={{ x: smoothX, y: smoothY, willChange: 'transform' }}
        >
          <motion.h1
            className="hero-stagger text-[4rem] sm:text-[6rem] md:text-[8rem] lg:text-[10rem] font-bold tracking-tighter leading-[0.85] text-text-primary uppercase"
            variants={heroStaggerVariants}
          >
            Avneesh
          </motion.h1>
        </motion.div>

        <motion.div
          className="parallax-layer overflow-hidden mb-8 flex flex-col md:flex-row items-center gap-4 md:gap-8"
          style={{ x: smoothX, y: smoothY, willChange: 'transform' }}
        >
          <motion.h1
            className="hero-stagger text-[4rem] sm:text-[6rem] md:text-[8rem] lg:text-[10rem] font-bold tracking-tighter leading-[0.85] text-gradient uppercase"
            variants={heroStaggerVariants}
          >
            Jadhav.
          </motion.h1>
        </motion.div>

        {/* Minimal Divider */}
        <motion.div
          className="hero-divider w-full max-w-2xl h-[1px] bg-text-primary/20 mb-10"
          style={{ transformOrigin: 'left center' }}
          variants={dividerVariants}
        />

        {/* Subtitle */}
        <motion.p
          className="hero-sub text-lg md:text-2xl text-text-secondary max-w-2xl leading-relaxed mb-12 font-light"
          variants={subVariants}
        >
          An <strong className="text-text-primary font-medium">AI Engineer and Full-Stack Developer</strong> focused on building AI-native applications, automation systems, and scalable software products.
        </motion.p>

        {/* CTAs */}
        <motion.div
          className="flex flex-wrap items-center justify-center gap-6"
          variants={ctaGroupVariants}
        >
          <motion.a
            href="#projects"
            className="hero-cta inline-flex items-center justify-center px-8 py-4 rounded-none border border-text-primary bg-text-primary text-surface font-mono-accent text-sm uppercase tracking-widest hover:bg-transparent hover:text-text-primary transition-colors duration-300"
            variants={ctaVariants}
          >
            View Projects
          </motion.a>
          <motion.a
            href="#contact"
            className="hero-cta inline-flex items-center justify-center px-8 py-4 rounded-none border border-text-primary/20 bg-transparent text-text-primary font-mono-accent text-sm uppercase tracking-widest hover:border-text-primary transition-colors duration-300"
            variants={ctaVariants}
          >
            Get in Touch
          </motion.a>
        </motion.div>
      </motion.div>

      {/* Scroll Indicator */}
      <motion.div
        className="hero-scroll absolute bottom-10 left-1/2 -translate-x-1/2 z-10"
        variants={scrollVariants}
        initial="hidden"
        animate="show"
        transition={{ delay: 1.95 }}
      >
        <div className="flex flex-col items-center gap-3 text-text-primary/50 hover:text-text-primary transition-colors">
          <span className="text-[10px] font-mono-accent uppercase tracking-[0.3em]">Scroll</span>
          <div className="w-[1px] h-12 bg-text-primary/30 relative overflow-hidden">
             <div className="absolute top-0 left-0 w-full h-full bg-text-primary animate-[shimmer_2s_infinite]" style={{ transformOrigin: 'top' }} />
          </div>
        </div>
      </motion.div>
    </section>
  );
}