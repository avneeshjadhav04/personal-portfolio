import { useEffect, useRef } from 'react';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import SectionGlow from './SectionGlow';

gsap.registerPlugin(ScrollTrigger);

export default function About() {
  const sectionRef = useRef<HTMLElement>(null);

  useEffect(() => {
    const ctx = gsap.context(() => {
      gsap.fromTo(
        '.about-image-container',
        { scale: 0.9, opacity: 0, rotationY: 5 },
        {
          scale: 1,
          opacity: 1,
          rotationY: 0,
          duration: 1.5,
          ease: 'power3.out',
          scrollTrigger: {
            trigger: sectionRef.current,
            start: 'top 60%',
          },
        }
      );

      gsap.fromTo(
        '.about-text-reveal',
        { y: 50, opacity: 0 },
        {
          y: 0,
          opacity: 1,
          duration: 1,
          ease: 'power3.out',
          stagger: 0.1,
          scrollTrigger: {
            trigger: sectionRef.current,
            start: 'top 60%',
          },
        }
      );
    }, sectionRef);

    return () => ctx.revert();
  }, []);

  return (
    <section id="about" ref={sectionRef} className="py-32 md:py-48 px-6 relative bg-background border-b border-border overflow-hidden">
      <SectionGlow color="#FF9933" position="top-right" size="lg" opacity={0.3} />
      <div className="max-w-7xl mx-auto relative z-10">
        <div className="grid lg:grid-cols-2 gap-16 lg:gap-24 items-start">
          {/* Left: Image */}
          <div className="about-image-container relative">
            <div className="aspect-[4/5] relative overflow-hidden bg-surface border border-text-primary/10 p-2">
              <img
                src="/avatar.jfif"
                alt="Avneesh Jadhav"
                className="w-full h-full object-cover filter grayscale hover:grayscale-0 transition-all duration-700"
                loading="lazy"
              />
            </div>
            {/* Minimal Accents */}
            <div className="absolute -bottom-4 -right-4 w-24 h-24 bg-gradient-to-br from-accent to-accent-glow blur-2xl opacity-50 pointer-events-none" />
          </div>

          {/* Right: Content */}
          <div className="flex flex-col justify-center h-full">
            <h2 className="about-text-reveal text-5xl md:text-7xl font-bold tracking-tighter text-text-primary mb-8 uppercase leading-[0.9]">
              Hello, I'm <span className="text-gradient">Avneesh.</span>
            </h2>
            
            <p className="about-text-reveal text-xl md:text-2xl text-text-secondary leading-relaxed mb-6 font-light">
              I am a <strong className="text-text-primary font-medium">Computer Engineering student</strong> at the University of Pune <span className="font-mono-accent text-xs bg-surface border border-border px-2 py-1 ml-2">(2022–2026)</span>.
            </p>
            
            <p className="about-text-reveal text-lg text-text-secondary leading-relaxed mb-12">
              My focus lies in Machine Learning, Deep Learning, and intelligent agentic systems. From healthcare claim assistance platforms to autonomous lead generation pipelines, I specialize in building robust AI-powered applications that make a tangible impact. My research was selected for presentation at the <strong className="text-text-primary font-medium">11th ICTIS conference in Bangkok, Thailand</strong>.
            </p>

            <div className="about-text-reveal grid grid-cols-2 gap-8 border-t border-border pt-8">
              <div>
                <span className="block text-[10px] font-mono-accent uppercase tracking-[0.2em] text-text-secondary mb-2">Location</span>
                <span className="text-lg font-medium text-text-primary">Pune, India</span>
              </div>
              <div>
                <span className="block text-[10px] font-mono-accent uppercase tracking-[0.2em] text-text-secondary mb-2">Focus</span>
                <span className="text-lg font-medium text-text-primary">AI & Systems</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
