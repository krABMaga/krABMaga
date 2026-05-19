#[test]
fn agentimpl_tracks_id_and_display() {
    use krabmaga::engine::agentimpl::AgentImpl;

    use crate::utils::mynode::MyNode;

    let agent = MyNode { id: 7, flag: true };
    let agent_impl = AgentImpl::new(Box::new(agent), 99);

    assert_eq!(agent_impl.to_string(), "99 false");
    assert!(agent_impl == AgentImpl::new(Box::new(MyNode { id: 7, flag: false }), 99));
    assert!(agent_impl != AgentImpl::new(Box::new(MyNode { id: 8, flag: true }), 100));
    assert_eq!(agent_impl.id(), 99);
}
