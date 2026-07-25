import { motion } from 'framer-motion';
import { Stethoscope, Bot, Phone } from 'lucide-react';
import TiltCard from './TiltCard';
import { easeStandard, easeSmooth, staggerContainer, viewportOnce } from '../lib/motion';

const featured = [
  {
    title: 'AI Healthcare Claims Assistance',
    description:
      'An AI-powered platform that automates healthcare insurance claims processing, reducing manual effort and improving accuracy using NLP and automation workflows.',
    tags: ['Python', 'Automation', 'NLP', 'n8n'],
    icon: Stethoscope,
    year: '2026',
    stat: '85%',
    statLabel: 'Manual Effort Reduced',
  },
  {
    title: 'Autonomous Lead Generation System',
    description:
      'A fully automated client outreach and lead generation system that identifies prospects, personalizes outreach, and manages follow-ups using AI agents.',
    tags: ['Automation', 'Python', 'APIs', 'n8n'],
    icon: Bot,
    year: '2026',
    stat: '3x',
    statLabel: 'Lead Volume Increase',
  },
  {
    title: 'Voice AI Call Handler',
    description:
      'A fully automated inbound call handling system using voice AI, capable of understanding caller intent, answering queries, and routing calls intelligently.',
    tags: ['Voice AI', 'Python', 'Twilio', 'STT/TTS'],
    icon: Phone,
    year: 'Feb 2025',
    stat: '24/7',
    statLabel: 'Always-On Coverage',
  },
];

const container = staggerContainer(0.1);

const cardVariant = {
  hidden: { opacity: 0, y: 50, scale: 0.95 },
  show: { opacity: 1, y: 0, scale: 1, transition: { duration: 0.7, ease: easeStandard } },
};

export default function FeaturedStack() {
  return (
    <section className="h-full flex flex-col justify-center px-6 relative">
      <div className="max-w-6xl mx-auto w-full">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={viewportOnce}
          transition={{ duration: 0.7, ease: easeSmooth }}
          className="text-center mb-12"
        >
          <h2 className="text-sm font-semibold text-accent-teal uppercase tracking-[0.2em] mb-4 font-mono-accent">
            Showcase
          </h2>
          <h3 className="text-3xl md:text-5xl font-bold mb-6 tracking-tight">
            Featured <span className="text-gradient">Work</span>
          </h3>
          <p className="text-text-secondary max-w-2xl mx-auto leading-relaxed">
            Three projects that represent the breadth of my capabilities in{' '}
            <span className="font-mono-accent text-accent-teal">AI automation</span> and intelligent systems.
          </p>
        </motion.div>

        <motion.div
          variants={container}
          initial="hidden"
          whileInView="show"
          viewport={viewportOnce}
          className="grid md:grid-cols-3 gap-6"
        >
          {featured.map((project) => (
            <motion.div key={project.title} variants={cardVariant}>
              <TiltCard className="h-full">
                <div className="group h-full p-6 md:p-8 rounded-2xl glass-card cursor-pointer flex flex-col">
                  <div className="flex items-start justify-between mb-6">
                    <div className="p-3 rounded-xl bg-accent-teal/10 border border-accent-teal/20">
                      <project.icon size={28} className="text-accent-teal" />
                    </div>
                    <span className="text-xs font-medium text-text-secondary bg-background/60 px-3 py-1 rounded-full border border-border font-mono-accent">
                      {project.year}
                    </span>
                  </div>

                  <div className="mb-6">
                    <span className="text-4xl md:text-5xl font-bold text-gradient font-mono-accent">
                      {project.stat}
                    </span>
                    <p className="text-sm text-text-secondary mt-1 font-mono-accent">
                      {project.statLabel}
                    </p>
                  </div>

                  <h4 className="text-xl font-bold text-white group-hover:text-gradient transition-all duration-300 mb-3">
                    {project.title}
                  </h4>

                  <p className="text-sm text-text-secondary leading-relaxed mb-6 flex-1">
                    {project.description}
                  </p>

                  <div className="flex flex-wrap gap-2">
                    {project.tags.map((tag) => (
                      <span
                        key={tag}
                        className="px-3 py-1 text-xs font-medium rounded-full bg-background/60 border border-border text-text-secondary font-mono-accent"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                </div>
              </TiltCard>
            </motion.div>
          ))}
        </motion.div>
      </div>
    </section>
  );
}
