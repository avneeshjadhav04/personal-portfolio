import { useEffect, useRef } from 'react';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

function SplitTextReveal({
  text,
  className = '',
  stagger = 0.03,
  delay = 0,
}: {
  text: string;
  className?: string;
  stagger?: number;
  delay?: number;
}) {
  const ref = useRef<HTMLParagraphElement>(null);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;

    const words = el.querySelectorAll('.split-word');
    const ctx = gsap.context(() => {
      gsap.fromTo(
        words,
        { y: 40, opacity: 0 },
        {
          y: 0,
          opacity: 1,
          duration: 0.8,
          ease: 'power3.out',
          stagger,
          delay,
          scrollTrigger: {
            trigger: el,
            start: 'top 80%',
          },
        }
      );
    });

    return () => ctx.revert();
  }, [stagger, delay]);

  const words = text.split(' ').map((word, i) => (
    <span key={i} className="split-word inline-block mr-[0.25em]">
      {word}
    </span>
  ));

  return (
    <p ref={ref} className={className}>
      {words}
    </p>
  );
}

export default function Philosophy() {
  const sectionRef = useRef<HTMLElement>(null);
  const bgRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const ctx = gsap.context(() => {
      if (bgRef.current) {
        gsap.to(bgRef.current, {
          yPercent: 20,
          ease: 'none',
          scrollTrigger: {
            trigger: sectionRef.current,
            start: 'top bottom',
            end: 'bottom top',
            scrub: true,
          },
        });
      }
    }, sectionRef);

    return () => ctx.revert();
  }, []);

  return (
    <section
      id="philosophy"
      ref={sectionRef}
      className="relative pt-20 md:pt-32 pb-20 md:pb-32 px-6 overflow-hidden bg-background"
    >
      {/* Parallax background texture */}
      <div
        ref={bgRef}
        className="absolute inset-0 z-0 scale-110 opacity-[0.04]"
        style={{
          backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)' opacity='1'/%3E%3C/svg%3E")`,
          backgroundSize: '200px 200px',
        }}
      />

      <div className="relative z-10 max-w-5xl mx-auto">
        <span className="text-[12px] font-mono-accent uppercase tracking-[0.3em] text-text-secondary mb-12 block border-l-2 border-accent pl-4">
          My Philosophy
        </span>

        <div className="space-y-12 md:space-y-16">
          <SplitTextReveal
            text="Most AI development focuses on: demos, proof-of-concepts, and blog posts that never see production."
            className="text-2xl md:text-4xl text-text-secondary leading-tight max-w-4xl font-light tracking-tight"
            stagger={0.02}
          />

          <div className="max-w-4xl">
            <SplitTextReveal
              text="I focus on:"
              className="text-xl md:text-2xl text-text-secondary leading-relaxed mb-6 font-mono-accent uppercase tracking-widest"
              stagger={0.03}
              delay={0.2}
            />
            <SplitTextReveal
              text="shipping systems that work."
              className="text-5xl sm:text-6xl md:text-8xl lg:text-[8rem] font-bold text-text-primary leading-[0.85] tracking-tighter uppercase"
              stagger={0.06}
              delay={0.4}
            />
          </div>
        </div>
      </div>
    </section>
  );
}
