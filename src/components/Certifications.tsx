import { motion } from 'framer-motion';
import {
  Award,
  GraduationCap,
  Cpu,
  Cloud,
  Database,
  BookOpen,
  Sparkles,
  Terminal,
} from 'lucide-react';
import TiltCard from './TiltCard';

const certifications = [
  {
    title: 'Machine Learning Specialization',
    org: 'Stanford University',
    skills: ['Supervised ML', 'Advanced Learning', 'Unsupervised Learning', 'Recommenders'],
    icon: GraduationCap,
    tint: 'from-red-500/5 to-orange-500/5',
  },
  {
    title: 'Fundamentals of Deep Learning',
    org: 'Nvidia',
    skills: ['Neural Networks', 'Deep Learning', 'GPU Acceleration'],
    icon: Cpu,
    tint: 'from-green-500/5 to-emerald-500/5',
  },
  {
    title: 'OCI Generative AI Professional',
    org: 'Oracle',
    skills: ['GenAI', 'Cloud AI', 'LLM Deployment'],
    icon: Cloud,
    tint: 'from-red-500/5 to-red-400/5',
  },
  {
    title: 'Oracle AI Vector Search Professional',
    org: 'Oracle',
    skills: ['Vector DB', 'RAG', 'Semantic Search'],
    icon: Database,
    tint: 'from-red-500/5 to-red-400/5',
  },
  {
    title: 'AMCAT Certified Software Engineer',
    org: 'IT Services',
    skills: ['Software Engineering', 'Problem Solving', 'Code Quality'],
    icon: Award,
    tint: 'from-blue-500/5 to-cyan-500/5',
  },
  {
    title: 'Model Context Protocol',
    org: 'Anthropic',
    skills: ['MCP', 'AI Integration', 'Tool Use'],
    icon: Sparkles,
    tint: 'from-amber-500/5 to-yellow-500/5',
  },
  {
    title: 'MCP Advanced Topics',
    org: 'Anthropic',
    skills: ['Advanced MCP', 'AI Architecture', 'System Design'],
    icon: BookOpen,
    tint: 'from-amber-500/5 to-yellow-500/5',
  },
  {
    title: 'Advanced Rust: Managing Projects',
    org: 'LinkedIn',
    skills: ['Rust', 'Project Management'],
    icon: Award,
    tint: 'from-orange-500/5 to-amber-500/5',
  },
  {
    title: 'Advanced Linux: The Linux Kernel',
    org: 'LinkedIn',
    skills: ['Linux', 'System Administration'],
    icon: Terminal,
    tint: 'from-emerald-500/5 to-teal-500/5',
  },
];

const container = {
  hidden: { opacity: 0 },
  show: {
    opacity: 1,
    transition: { staggerChildren: 0.08 },
  },
};

const cardVariant = {
  hidden: { opacity: 0, y: 30, scale: 0.95 },
  show: { opacity: 1, y: 0, scale: 1, transition: { duration: 0.5 } },
};

export default function Certifications() {
  return (
    <section id="certifications" className="pt-32 pb-24 md:pt-40 md:pb-32 px-6 relative">
      <div className="max-w-6xl mx-auto">
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true }}
          transition={{ duration: 0.7 }}
          className="text-center mb-16"
        >
          <h2 className="text-sm font-semibold text-accent-teal uppercase tracking-[0.2em] mb-4 font-mono-accent">
            Credentials
          </h2>
          <h3 className="text-3xl md:text-5xl font-bold mb-6 tracking-tight">Certifications</h3>
          <p className="text-text-secondary max-w-2xl mx-auto leading-relaxed">
            Industry-recognized certifications from <span className="font-mono-accent text-accent-teal">Stanford</span>,
            <span className="font-mono-accent text-accent-teal"> Nvidia</span>,
            <span className="font-mono-accent text-accent-teal"> Oracle</span>,
            <span className="font-mono-accent text-accent-teal"> LinkedIn</span>, and
            <span className="font-mono-accent text-accent-teal"> Anthropic</span>.
          </p>
        </motion.div>

        <motion.div
          variants={container}
          initial="hidden"
          whileInView="show"
          viewport={{ once: true, margin: '-50px' }}
          className="grid md:grid-cols-2 lg:grid-cols-3 gap-6"
        >
          {certifications.map((cert) => (
            <motion.div key={cert.title} variants={cardVariant}>
              <TiltCard className="h-full">
                <div className={`group h-full p-6 rounded-2xl bg-surface bg-gradient-to-br ${cert.tint} border border-border transition-all duration-300 hover:-translate-y-1 hover:border-accent-teal/40 hover:shadow-lg`}>
                  <div className="flex items-start justify-between mb-5">
                    <div className="p-3 rounded-xl bg-surface-light border border-border">
                      <cert.icon size={24} className="text-accent-teal" />
                    </div>
                    <span className="text-xs font-medium text-text-secondary bg-surface-light px-3 py-1 rounded-full border border-border font-mono-accent">
                      {cert.org}
                    </span>
                  </div>

                  <h4 className="text-lg font-bold text-text-primary mb-4">
                    {cert.title}
                  </h4>

                  <div className="flex flex-wrap gap-2">
                    {cert.skills.map((skill) => (
                      <span
                        key={skill}
                        className="px-2.5 py-1 text-xs font-medium rounded-full bg-surface-light border border-border text-text-secondary font-mono-accent"
                      >
                        {skill}
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
