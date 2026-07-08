import { useEffect, useRef } from 'react';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';
import SectionGlow from './SectionGlow';

gsap.registerPlugin(ScrollTrigger);



/* ============================================
   YOUTUBE EMBED COMPONENT
   ============================================ */
function YouTubeEmbed({ videoId }: { videoId: string }) {
  return (
    <div className="w-full h-full flex items-center justify-center p-4 md:p-8">
      <div className="w-full aspect-video bg-black">
        <iframe
          src={`https://www.youtube-nocookie.com/embed/${videoId}`}
          title="YouTube video player"
          allow="accelerometer; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
          referrerPolicy="strict-origin-when-cross-origin"
          allowFullScreen
          loading="lazy"
          className="w-full h-full border-0"
        />
      </div>
    </div>
  );
}

/* ============================================
    FEATURED PROJECTS  Normal scroll cards
   ============================================ */
const featuredProjects = [
  {
    step: '01',
    title: 'Project Vulcan: AI Assistant',
    description:
      'An open-source platform that helps you operate AI with terminal-level access for your daily workflows, making them simpler, more secure, and self-hosted.',
    tags: ['TypeScript', 'React', 'Rust', 'Axum', 'SQLite', 'AI'],
    visual: <YouTubeEmbed videoId="LhgXK4dvE3w" />,
    link: 'https://project-vulcan.onrender.com/',
  },
  {
    step: '02',
    title: 'Kovero AI: AI Powered Healthcare Claims Assistance Platform',
    description:
      'A full-fledged user-centric insurance claims assistance platform that simplifies finding the right health policies and helps users prepare for claims. Integrates AI using OCR and transformer-based LLMs for document verification and query resolution.',
    tags: ['Next.js', 'React', 'TypeScript', 'Rust', 'Axum', 'SQLite', 'Docker', 'AI'],
    visual: <YouTubeEmbed videoId="9f2AL8xu1NU" />,
    link: 'https://koveroai-alpha.onrender.com/',
  },
      {
      step: '03',
      title: 'LLM From Scratch',
      description: '124M parameter language model trained from scratch on 2B tokens. Built every layer in PyTorch, no Trainer.train(). Validation perplexity 14.8, trained in 5 hours. Live API, weights, and code available.',
      tags: ['PyTorch', 'LLM', 'Transformers', 'AI', 'NLP'],
      visual: <YouTubeEmbed videoId="e3es5UZgxq0" />,
      link: 'https://avneeshjadhav04--llm-api.modal.run/',
    },
];

export default function FeaturedProjects() {
  const sectionRef = useRef<HTMLElement>(null);
  const cardsRef = useRef<HTMLDivElement[]>([]);

  useEffect(() => {
    const cards = cardsRef.current.filter(Boolean);
    if (cards.length === 0) return;

    const ctx = gsap.context(() => {
      // Simple fade-in + slide-up on scroll for each card
      cards.forEach((card) => {
        gsap.fromTo(
          card,
          { y: 80, opacity: 0 },
          {
            y: 0,
            opacity: 1,
            duration: 1,
            ease: 'power3.out',
            scrollTrigger: {
              trigger: card,
              start: 'top 85%',
              toggleActions: 'play none none none',
            },
          }
        );
      });
    }, sectionRef);

    return () => ctx.revert();
  }, []);

  return (
    <section id="projects" ref={sectionRef} className="relative bg-background overflow-hidden">
      <SectionGlow color="#FF3366" position="center" size="xl" opacity={0.2} />
      <div className="py-24 md:py-32 px-6 relative z-10">
        <div className="max-w-6xl mx-auto text-center mb-16">
          <span className="text-[12px] font-mono-accent uppercase tracking-[0.3em] text-text-secondary mb-4 block">
            Portfolio
          </span>
          <h2 className="text-5xl md:text-7xl font-bold tracking-tighter text-text-primary mb-4 uppercase">
            Featured <span className="text-gradient">Projects.</span>
          </h2>
          <p className="text-xl text-text-secondary max-w-xl mx-auto leading-relaxed font-light">
            Real-world systems solving real problems using AI.
          </p>
        </div>

        {/* Normal stacked cards */}
        <div className="max-w-5xl mx-auto space-y-12 md:space-y-16">
          {featuredProjects.map((project, i) => (
            <div
              key={project.step}
              ref={(el) => {
                if (el) cardsRef.current[i] = el;
              }}
              className="rounded-none bg-surface border border-border overflow-hidden shadow-2xl opacity-0"
            >
              <div className="grid md:grid-cols-2 gap-0">
                {/* Visual Side */}
                <div className="relative h-64 md:h-auto md:min-h-[480px] flex items-center justify-center overflow-hidden border-b md:border-b-0 md:border-r border-border">
                  {project.visual}
                </div>

                {/* Content Side */}
                <div className="p-8 md:p-12 flex flex-col justify-center">
                  <span className="text-[10px] font-mono-accent uppercase tracking-[0.25em] text-text-secondary mb-4 block">
                    Project {project.step}
                  </span>
                  <h3 className="text-3xl md:text-4xl font-bold text-text-primary mb-6 tracking-tight uppercase">
                    {project.title}
                  </h3>
                  <p className="text-lg text-text-secondary leading-relaxed mb-8 font-light">
                    {project.description}
                  </p>
                  <div className="flex flex-wrap gap-2 mb-8">
                    {project.tags.map((tag) => (
                      <span
                        key={tag}
                        className="px-3 py-1.5 text-xs font-mono-accent font-medium bg-surface border border-text-primary/20 text-text-primary uppercase tracking-wider"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                  {project.link && (
                    <a
                      href={project.link}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="inline-flex items-center justify-center px-8 py-4 bg-text-primary text-surface font-mono-accent text-sm uppercase tracking-widest hover:bg-accent transition-colors duration-300"
                    >
                      View Project
                    </a>
                  )}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
