pub mod about;
pub mod certifications;
pub mod contact;
pub mod experience;
pub mod featured_projects;
pub mod footer;
pub mod hero;
pub mod icons;
pub mod navbar;
pub mod philosophy;
pub mod preloader;
pub mod projects;
pub mod scroll_progress;
pub mod section_glow;
pub mod skills;
pub mod smooth_scroll;
pub mod tilt_card;

pub use about::About;
pub use certifications::Certifications;
pub use contact::Contact;
pub use experience::ExperienceSection;
pub use featured_projects::FeaturedProjects;
pub use footer::Footer;
pub use hero::Hero;
pub use icons::{
    Award as AwardIcon, Bot as BotIcon, BookOpen as BookOpenIcon, Box as BoxIcon,
    Brain as BrainIcon, Briefcase as BriefcaseIcon, Calendar as CalendarIcon, Cloud as CloudIcon,
    Cpu as CpuIcon, CreditCard as CreditCardIcon, Database as DatabaseIcon,
    EmailIcon, GitHubIcon, GitBranch as GitBranchIcon, Globe as GlobeIcon,
    GraduationCap as GraduationCapIcon, Home as HomeIcon, Languages as LanguagesIcon,
    Layers as LayersIcon, LinkedInIcon, MapPin as MapPinIcon, Menu as MenuIcon,
    Network as NetworkIcon, Package as PackageIcon, Phone as PhoneIcon,
    RefreshCw as RefreshCwIcon, Server as ServerIcon, Sparkles as SparklesIcon,
    Stethoscope as StethoscopeIcon, Terminal as TerminalIcon, Wrench as WrenchIcon,
    X as XIcon,
};
pub use navbar::Navbar;
pub use philosophy::Philosophy;
pub use preloader::Preloader;
pub use projects::Projects;
pub use scroll_progress::ScrollProgress;
pub use section_glow::{GlowPosition, GlowSize, SectionGlow};
pub use skills::Skills;
pub use smooth_scroll::{ScrollTarget, SmoothScrollProvider, use_smooth_scroll};
pub use tilt_card::TiltCard;