import { useEffect, useRef } from 'react';
import gsap from 'gsap';

export default function Hero() {
  const sectionRef = useRef<HTMLElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const ctx = gsap.context(() => {
      // Cinematic timeline
      const tl = gsap.timeline({ defaults: { ease: 'power4.out' } });

      tl.fromTo(
        '.hero-stagger',
        { y: 150, opacity: 0, rotateZ: 2 },
        { y: 0, opacity: 1, rotateZ: 0, duration: 1.5, stagger: 0.15 }
      )
        .fromTo(
          '.hero-divider',
          { scaleX: 0, transformOrigin: 'left center' },
          { scaleX: 1, duration: 1.2, ease: 'power3.inOut' },
          '-=1.0'
        )
        .fromTo(
          '.hero-sub',
          { y: 30, opacity: 0 },
          { y: 0, opacity: 1, duration: 1 },
          '-=0.8'
        )
        .fromTo(
          '.hero-cta',
          { y: 20, opacity: 0 },
          { y: 0, opacity: 1, duration: 0.8, stagger: 0.1 },
          '-=0.6'
        )
        .fromTo(
          '.hero-scroll',
          { opacity: 0, y: -20 },
          { opacity: 1, y: 0, duration: 1 },
          '-=0.4'
        );

      // Subtle parallax on mouse move
      const handleMouseMove = (e: MouseEvent) => {
        const { clientX, clientY } = e;
        const centerX = window.innerWidth / 2;
        const centerY = window.innerHeight / 2;
        
        const moveX = (clientX - centerX) * 0.02;
        const moveY = (clientY - centerY) * 0.02;

        gsap.to('.parallax-layer', {
          x: moveX,
          y: moveY,
          duration: 1,
          ease: 'power2.out',
        });
      };

      window.addEventListener('mousemove', handleMouseMove);
      return () => window.removeEventListener('mousemove', handleMouseMove);

    }, sectionRef);

    return () => ctx.revert();
  }, []);

  return (
    <section
      ref={sectionRef}
      className="relative w-full min-h-[100dvh] flex items-center justify-center overflow-hidden pt-20"
    >
      <div
        ref={containerRef}
        className="relative z-10 w-full max-w-7xl mx-auto px-6 md:px-10 flex flex-col items-center text-center"
      >
        {/* Massive Typography Group */}
        <div className="parallax-layer overflow-hidden mb-2">
          <h1 className="hero-stagger text-[4rem] sm:text-[6rem] md:text-[8rem] lg:text-[10rem] font-bold tracking-tighter leading-[0.85] text-text-primary uppercase">
            Avneesh
          </h1>
        </div>
        
        <div className="parallax-layer overflow-hidden mb-8 flex flex-col md:flex-row items-center gap-4 md:gap-8">
          <h1 className="hero-stagger text-[4rem] sm:text-[6rem] md:text-[8rem] lg:text-[10rem] font-bold tracking-tighter leading-[0.85] text-gradient uppercase">
            Jadhav.
          </h1>
        </div>

        {/* Minimal Divider */}
        <div className="hero-divider w-full max-w-2xl h-[1px] bg-text-primary/20 mb-10" />

        {/* Subtitle */}
        <p className="hero-sub text-lg md:text-2xl text-text-secondary max-w-2xl leading-relaxed mb-12 font-light">
          An <strong className="text-text-primary font-medium">AI Engineer and Full-Stack Developer</strong> focused on building AI-native applications, automation systems, and scalable software products.
        </p>

        {/* CTAs */}
        <div className="flex flex-wrap items-center justify-center gap-6">
          <a
            href="#projects"
            className="hero-cta inline-flex items-center justify-center px-8 py-4 rounded-none border border-text-primary bg-text-primary text-surface font-mono-accent text-sm uppercase tracking-widest hover:bg-transparent hover:text-text-primary transition-colors duration-300"
          >
            View Projects
          </a>
          <a
            href="#contact"
            className="hero-cta inline-flex items-center justify-center px-8 py-4 rounded-none border border-text-primary/20 bg-transparent text-text-primary font-mono-accent text-sm uppercase tracking-widest hover:border-text-primary transition-colors duration-300"
          >
            Get in Touch
          </a>
        </div>
      </div>

      {/* Scroll Indicator */}
      <div className="hero-scroll absolute bottom-10 left-1/2 -translate-x-1/2 z-10">
        <div className="flex flex-col items-center gap-3 text-text-primary/50 hover:text-text-primary transition-colors">
          <span className="text-[10px] font-mono-accent uppercase tracking-[0.3em]">Scroll</span>
          <div className="w-[1px] h-12 bg-text-primary/30 relative overflow-hidden">
             <div className="absolute top-0 left-0 w-full h-full bg-text-primary animate-[shimmer_2s_infinite]" style={{ transformOrigin: 'top' }} />
          </div>
        </div>
      </div>
    </section>
  );
}
